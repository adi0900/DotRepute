import { ScoreItem } from "@/components/dashboard/ScoreItem";
import { Activity, Coins, Shield, Vote } from "lucide-react";

export function ReputationDataDisplay({
  data,
  theme,
  content,
}: {
  data: any;
  theme: "light" | "dark";
  content: string;
}) {
  // Check if this is a full reputation score response with all required fields
  const hasFullBreakdown =
    data?.breakdown?.identity &&
    data?.breakdown?.governance &&
    data?.breakdown?.staking &&
    data?.breakdown?.activity;

  return (
    <div className="space-y-4">
      <p className="text-sm whitespace-pre-line">{content}</p>

      {/* Only show score overview if we have totalScore and maxScore */}
      {data?.totalScore !== undefined && data?.maxScore !== undefined && (
        <div className="space-y-2">
          <div className="flex items-baseline gap-3">
            <span className="text-4xl font-bold">{data.totalScore}</span>
            <span
              className={theme === "light" ? "text-gray-600" : "text-gray-500"}
            >
              / {data.maxScore}
            </span>
          </div>

          <div
            className={`border h-2 overflow-hidden ${
              theme === "light"
                ? "border-black/20 bg-gray-100"
                : "border-white/10 bg-white/5"
            }`}
          >
            <div
              className={`h-full ${
                theme === "light"
                  ? "bg-gradient-to-r from-orange-500 to-yellow-500"
                  : "bg-gradient-to-r from-orange-400 to-yellow-400"
              }`}
              style={{ width: `${(data.totalScore / data.maxScore) * 100}%` }}
            />
          </div>

          {/* Only show rank/percentile if available */}
          {(data?.rank || data?.percentile) && (
            <div className="flex gap-4 text-sm">
              {data.rank && (
                <span
                  className={
                    theme === "light" ? "text-gray-600" : "text-gray-500"
                  }
                >
                  Rank: <span className="font-bold">#{data.rank}</span>
                </span>
              )}
              {data.percentile && (
                <span
                  className={
                    theme === "light" ? "text-gray-600" : "text-gray-500"
                  }
                >
                  {data.percentile}
                </span>
              )}
            </div>
          )}
        </div>
      )}

      {/* Breakdown - only show if all components exist */}
      {hasFullBreakdown && (
        <div className="grid grid-cols-2 gap-3 pt-2">
          <ScoreItem
            icon={<Shield className="w-4 h-4" />}
            label="Identity"
            score={data.breakdown.identity.score}
            max={data.breakdown.identity.max}
            theme={theme}
          />
          <ScoreItem
            icon={<Vote className="w-4 h-4" />}
            label="Governance"
            score={data.breakdown.governance.score}
            max={data.breakdown.governance.max}
            theme={theme}
          />
          <ScoreItem
            icon={<Coins className="w-4 h-4" />}
            label="Staking"
            score={data.breakdown.staking.score}
            max={data.breakdown.staking.max}
            theme={theme}
          />
          <ScoreItem
            icon={<Activity className="w-4 h-4" />}
            label="Activity"
            score={data.breakdown.activity.score}
            max={data.breakdown.activity.max}
            theme={theme}
          />
        </div>
      )}
    </div>
  );
}
