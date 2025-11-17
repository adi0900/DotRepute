import { HttpErrors, HttpStatusCode } from "@/helpers/Http.ts";
import { apiResponse } from "@/helpers/response.ts";
import { asyncHandler } from "@/helpers/request.ts";
import axios from "axios";
import type { Request, Response } from "express";
import { config } from "../../config.ts";

/**
 * GET /reputation/score/:address
 * Fetches reputation score for a specific Polkadot address
 * Integrates data from governance, staking, and identity
 */
export const getReputationScore = asyncHandler(async (req: Request, res: Response) => {
    const { address } = req.params;

    if (!address) {
        const resErr = apiResponse.error(HttpErrors.BadRequest("Address is required"));
        res.status(resErr.code).send(resErr);
        return;
    }

    try {
        // Fetch governance participation
        const governanceResponse = await axios.post(
            "https://assethub-polkadot.api.subscan.io/api/scan/account/referendum",
            {
                account: address,
                page: 0,
                row: 100,
            },
            {
                headers: {
                    "Content-Type": "application/json",
                    "x-api-key": config.subscanApiKey,
                },
            }
        );

        // Fetch staking info
        const stakingResponse = await axios.post(
            "https://polkadot.api.subscan.io/api/v2/scan/account/tokens",
            {
                address: address,
            },
            {
                headers: {
                    "Content-Type": "application/json",
                    "x-api-key": config.subscanApiKey,
                },
            }
        );

        // Calculate scores based on Rust reputation.rs logic
        const governanceData = governanceResponse.data;
        const stakingData = stakingResponse.data;

        // Governance Score (max 100)
        const referendaCount = governanceData?.data?.list?.length || 0;
        const governanceScore = Math.min(100, Math.floor((referendaCount / 20) * 100));

        // Staking Score (max 100)
        const nativeTokens = stakingData?.data?.native || [];
        const totalStaked = nativeTokens.reduce((sum: number, token: any) => {
            return sum + (parseFloat(token.balance) / Math.pow(10, token.decimals || 10));
        }, 0);
        const stakingScore = Math.min(100, Math.floor((totalStaked / 1000) * 100));

        // Identity Score (max 100) - simplified for now
        const identityScore = 85; // Will be calculated from identity pallet

        // Activity Score (max 100)
        const activityScore = Math.min(100, Math.floor((referendaCount + (totalStaked > 0 ? 20 : 0)) * 0.8));

        // Total Reputation Score (weighted average)
        const totalScore = Math.floor(
            (identityScore * 0.25) +
            (governanceScore * 0.25) +
            (stakingScore * 0.20) +
            (activityScore * 0.20) +
            (10 * 0.10) // Dev score placeholder
        );

        const result = {
            address,
            totalScore,
            maxScore: 100,
            breakdown: {
                identity: { score: identityScore, max: 100 },
                governance: { score: governanceScore, max: 100 },
                staking: { score: stakingScore, max: 100 },
                activity: { score: activityScore, max: 100 },
            },
            details: {
                referendaVoted: referendaCount,
                totalStaked: totalStaked.toFixed(4),
                lastUpdated: new Date().toISOString(),
            },
        };

        const resSuccess = apiResponse.success(
            HttpStatusCode.OK,
            result,
            "Reputation score calculated successfully"
        );
        res.status(resSuccess.code).send(resSuccess);

    } catch (err: any) {
        console.error("Reputation calculation error:", err.response?.data || err.message);

        const status = err.response?.status || 500;
        const message = err.response?.data?.message || "Failed to calculate reputation score";

        const resErr = apiResponse.error(HttpErrors.BadRequest(message));
        res.status(status).send(resErr);
    }
});
