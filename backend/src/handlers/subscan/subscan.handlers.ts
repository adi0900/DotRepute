import { HttpErrors, HttpStatusCode } from "@/helpers/Http.ts";
import { apiResponse } from "@/helpers/response.ts";
import { asyncHandler } from "@/helpers/request.ts";
import axios from "axios";
import type { Request, Response } from "express";
import { config } from "../../config.ts";

/**
 * GET /subscan/account/referenda
 * Fetches the list of referenda associated with a specific account from the Subscan API.
*/

export const getAccountReferenda = asyncHandler(async (req: Request, res: Response) => {
    const { account, module, page, row } = req.query;

    try {
        const response = await axios.post(
            "https://assethub-polkadot.api.subscan.io/api/scan/account/referendum",
            {
                account,
                module,
                page: Number(page),
                row: Number(row),
            },
            {
                headers: {
                    "Content-Type": "application/json",
                    "x-api-key": config.subscanApiKey,
                },
            }
        );

        const data = response.data;

        if (!data || data.code !== 0) {
            const resErr = apiResponse.error(HttpErrors.BadRequest("Failed to fetch account referenda list from Subscan"));
            res.status(resErr.code).send(resErr);
            return;
        }

        const listReferenda = [];

        for (const activity of data.data.list) {
            const referendumIndex = activity.referendum_index;

            const [titleResponse, statusResponse]  = await Promise.all([
                await axios.post(
                    "https://assethub-polkadot.api.subscan.io/api/scan/governance/desc",
                    {
                        gov: "referendums_v2",
                        id: String(referendumIndex)
                    },
                    {
                        headers: {
                            "Content-Type": "application/json",
                            "x-api-key": config.subscanApiKey,
                        },
                    }
                ),

                await axios.post(
                    "https://assethub-polkadot.api.subscan.io/api/scan/referenda/votes",
                    {
                        account,
                        order: "asc",
                        page: Number(page),
                        referendum_index: Number(referendumIndex),
                        row: Number(row),
                        sort: "conviction",
                        valid: "valid",
                    },
                    {
                        headers: {
                            "Content-Type": "application/json",
                            "x-api-key": config.subscanApiKey,
                        },
                    }
                )
            ]);

            const titleData = titleResponse.data.data.title;
            const statusData = statusResponse.data.data.list[0].status;

            listReferenda.push(
                `Referendum ${referendumIndex} - ${titleData} (${statusData})`
            );
        }

        const countList = listReferenda.length;
        const result = {
            count: countList,
            referenda: listReferenda
        };

        const resSuccess = apiResponse.success(HttpStatusCode.OK, result, "Fetched account referenda list successfully");
        res.status(resSuccess.code).send(resSuccess);

    } catch (err: any) {
        console.error("Subscan API error:", err.response?.data || err.message);

        const status = err.response?.status || 500;
        const message = err.response?.data?.message || "Failed to fetch account referenda list from Subscan";

        const resErr = apiResponse.error(HttpErrors.BadRequest(message));
        res.status(status).send(resErr);
    }
});