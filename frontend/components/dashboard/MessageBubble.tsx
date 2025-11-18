import { ReputationDataDisplay } from "@/components/dashboard/ReputationDataDisplay";
import { Message } from "@/types/default-types";
import { Bookmark, Bot, User } from "lucide-react";

// Message Bubble Component
export function MessageBubble({
  message,
  theme,
  onToggleBookmark,
}: {
  message: Message;
  theme: "light" | "dark";
  onToggleBookmark: () => void;
}) {
  const isUser = message.role === "user";

  return (
    <div
      className={`flex items-start gap-3 ${isUser ? "flex-row-reverse" : ""}`}
    >
      {/* Avatar */}
      <div
        className={`border p-2 flex-shrink-0 ${
          theme === "light"
            ? "border-black/20 bg-white"
            : "border-white/10 bg-black/40"
        }`}
      >
        {isUser ? <User className="w-5 h-5" /> : <Bot className="w-5 h-5" />}
      </div>

      {/* Message Content */}
      <div
        className={`flex-1 min-w-0 ${isUser ? "flex flex-col items-end" : ""}`}
      >
        <div
          className={`border px-4 py-3 w-full ${
            isUser
              ? theme === "light"
                ? "border-black/20 bg-orange-50"
                : "border-orange-900/30 bg-orange-950/20"
              : theme === "light"
                ? "border-black/10 bg-white"
                : "border-white/5 bg-black/20"
          }`}
        >
          {message.type === "data" && message.data ? (
            <ReputationDataDisplay
              data={message.data}
              theme={theme}
              content={message.content}
            />
          ) : (
            <p className="text-sm leading-relaxed whitespace-pre-line break-words">
              {message.content}
            </p>
          )}

          {/* Message Actions */}
          {!isUser && (
            <div className="flex items-center gap-2 mt-3 pt-3 border-t border-current border-opacity-10">
              <button
                onClick={onToggleBookmark}
                className={`transition-colors ${
                  message.isBookmarked
                    ? "text-yellow-500"
                    : theme === "light"
                      ? "text-gray-400 hover:text-gray-700"
                      : "text-gray-600 hover:text-gray-400"
                }`}
              >
                <Bookmark
                  className={`w-4 h-4 ${message.isBookmarked ? "fill-current" : ""}`}
                />
              </button>
            </div>
          )}
        </div>

        {/* Timestamp */}
        <span
          className={`text-xs font-mono mt-1 ${
            theme === "light" ? "text-gray-500" : "text-gray-600"
          }`}
        >
          {message.timestamp.toLocaleTimeString([], {
            hour: "2-digit",
            minute: "2-digit",
          })}
        </span>
      </div>
    </div>
  );
}
