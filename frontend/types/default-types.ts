export interface Message {
  id: string;
  role: "user" | "bot";
  content: string;
  timestamp: Date;
  type?: "text" | "data" | "question";
  data?: any;
  isBookmarked?: boolean;
}

// Chat Session type
export interface ChatSession {
  id: string;
  title: string;
  lastMessage: string;
  timestamp: Date;
  messageCount: number;
}
