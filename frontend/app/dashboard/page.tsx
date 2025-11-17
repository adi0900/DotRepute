/**
 * DotRepute Dashboard Page - Interactive Chat Interface with Sidebar
 * Venture-style dashboard with bot conversation flow
 * Features: Chat history, Bookmarks, Expandable messages
 */

"use client";

import { useEffect, useState, useRef } from "react";
import { VentureNavbar } from "@/components/venture-navbar";
import { useAccount } from "@luno-kit/react";
import { PolkadotInfrastructure, NETWORKS } from "@/lib/polkadot-api";
import Link from "next/link";
import {
  Send,
  Bot,
  User,
  Sparkles,
  ChevronRight,
  Shield,
  Vote,
  Coins,
  Activity,
  TrendingUp,
  Bookmark,
  MessageSquare,
  Plus,
  Star,
  Clock,
  LogIn,
  UserPlus,
  Trash2,
  MoreVertical,
  Download,
  Trophy,
  TrendingDown,
  Lightbulb,
} from "lucide-react";
import { Document, Paragraph, TextRun, Packer } from "docx";
import { saveAs } from "file-saver";
import { extractPolkadotAddress } from "@/lib/extractPolkadotAddress";

// Message type definition
interface Message {
  id: string;
  role: "user" | "bot";
  content: string;
  timestamp: Date;
  type?: "text" | "data" | "question";
  data?: any;
  isBookmarked?: boolean;
}

// Chat Session type
interface ChatSession {
  id: string;
  title: string;
  lastMessage: string;
  timestamp: Date;
  messageCount: number;
}

export default function DashboardPage() {
  const [theme, setTheme] = useState<"light" | "dark">("dark");
  const [mounted, setMounted] = useState(false);
  const [messages, setMessages] = useState<Message[]>([]);
  const [inputValue, setInputValue] = useState("");
  const [isTyping, setIsTyping] = useState(false);
  const [isLoggedIn, setIsLoggedIn] = useState(true); // Default to logged in on dashboard
  const [sidebarTab, setSidebarTab] = useState<"chats" | "bookmarks">("chats");
  const [chatSessions, setChatSessions] = useState<ChatSession[]>([]);
  const [governanceData, setGovernanceData] = useState<any>(null);
  const [currentSessionId, setCurrentSessionId] = useState<string>("1");
  const [sessionMessages, setSessionMessages] = useState<
    Record<string, Message[]>
  >({});
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const { address } = useAccount();
  const [polkadotApi, setPolkadotApi] = useState<PolkadotInfrastructure | null>(
    null
  );

  // Initialize Polkadot API connection
  useEffect(() => {
    const initApi = async () => {
      try {
        const api = new PolkadotInfrastructure(NETWORKS.POLKADOT);
        await api.connect();
        setPolkadotApi(api);
      } catch (error) {
        console.error("Failed to connect to Polkadot:", error);
      }
    };

    initApi();

    return () => {
      if (polkadotApi) {
        polkadotApi.disconnect();
      }
    };
  }, []);

  // Initialize theme from localStorage on mount
  useEffect(() => {
    setMounted(true);
    const savedTheme = localStorage.getItem("theme") as "light" | "dark" | null;
    const initialTheme = savedTheme || "dark";
    setTheme(initialTheme);
    document.documentElement.classList.toggle("dark", initialTheme === "dark");

    // Load saved chat sessions from localStorage
    const savedSessions = localStorage.getItem("chatSessions");
    const savedSessionMessages = localStorage.getItem("sessionMessages");
    const savedCurrentSession = localStorage.getItem("currentSessionId");

    if (savedSessions && savedSessionMessages) {
      const sessions = JSON.parse(savedSessions);
      const messagesMap = JSON.parse(savedSessionMessages);

      // Parse dates back from strings
      sessions.forEach((session: ChatSession) => {
        session.timestamp = new Date(session.timestamp);
      });

      Object.keys(messagesMap).forEach((sessionId) => {
        messagesMap[sessionId].forEach((msg: Message) => {
          msg.timestamp = new Date(msg.timestamp);
        });
      });

      setChatSessions(sessions);
      setSessionMessages(messagesMap);

      const currentId = savedCurrentSession || sessions[0]?.id || "1";
      setCurrentSessionId(currentId);
      setMessages(messagesMap[currentId] || []);
    } else {
      // Initial bot greeting for new users
      const initialMessage: Message = {
        id: "1",
        role: "bot",
        content: address
          ? `Welcome to DotRepute! I'm your reputation assistant. I can help you check your reputation score, analyze contributions, view governance participation, and more.\n\nYour wallet is connected: ${address.slice(0, 6)}...${address.slice(-4)}\n\nTry asking:\n• "Show my reputation score"\n• "What's my governance activity?"\n• "Show my staking info"\n• "What can you help me with?"`
          : "Welcome to DotRepute! I'm your reputation assistant.\n\nPlease connect your wallet using the button in the top navigation to view your personalized reputation scores and blockchain activity.",
        timestamp: new Date(),
        type: "text",
      };

      const initialSession: ChatSession = {
        id: "1",
        title: "New Chat",
        lastMessage: "Welcome to DotRepute!",
        timestamp: new Date(),
        messageCount: 1,
      };

      setMessages([initialMessage]);
      setChatSessions([initialSession]);
      setSessionMessages({ "1": [initialMessage] });
    }
  }, []);

  // Auto-scroll to bottom when new messages arrive
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  // Save chat sessions to localStorage whenever they change
  useEffect(() => {
    if (mounted && chatSessions.length > 0) {
      localStorage.setItem("chatSessions", JSON.stringify(chatSessions));
      localStorage.setItem("sessionMessages", JSON.stringify(sessionMessages));
      localStorage.setItem("currentSessionId", currentSessionId);
    }
  }, [chatSessions, sessionMessages, currentSessionId, mounted]);

  // Toggle theme and persist to localStorage
  const toggleTheme = () => {
    const newTheme = theme === "dark" ? "light" : "dark";
    setTheme(newTheme);
    localStorage.setItem("theme", newTheme);
    document.documentElement.classList.toggle("dark", newTheme === "dark");
  };

  // Handle logout
  const handleLogout = () => {
    // Optionally clear chat history on logout
    // localStorage.removeItem('chatSessions');
    // localStorage.removeItem('sessionMessages');
    // localStorage.removeItem('currentSessionId');

    setIsLoggedIn(false);
    // Redirect to auth page
    window.location.href = "/auth";
  };

  // Create new chat session
  const createNewChat = () => {
    const newSessionId = Date.now().toString();
    const initialMessage: Message = {
      id: Date.now().toString(),
      role: "bot",
      content: "Hello! How can I assist you with your reputation today?",
      timestamp: new Date(),
      type: "text",
    };

    const newSession: ChatSession = {
      id: newSessionId,
      title: "New Chat",
      lastMessage: "",
      timestamp: new Date(),
      messageCount: 1,
    };

    setChatSessions((prev) => [newSession, ...prev]);
    setCurrentSessionId(newSessionId);
    setMessages([initialMessage]);
    setSessionMessages((prev) => ({
      ...prev,
      [newSessionId]: [initialMessage],
    }));
  };

  // Switch to a different chat session
  const switchSession = (sessionId: string) => {
    setCurrentSessionId(sessionId);
    setMessages(sessionMessages[sessionId] || []);
  };

  // Toggle bookmark on message
  const toggleBookmark = (messageId: string) => {
    setMessages((prev) =>
      prev.map((msg) =>
        msg.id === messageId ? { ...msg, isBookmarked: !msg.isBookmarked } : msg
      )
    );
  };

  // Get bookmarked messages
  const bookmarkedMessages = messages.filter((msg) => msg.isBookmarked);

  // Download chat as Word document with complete conversation data
  const downloadChatAsWord = async () => {
    try {
      // Get current session info
      const currentSession = chatSessions.find(
        (s) => s.id === currentSessionId
      );
      const sessionTitle = currentSession?.title || "Chat Session";

      const doc = new Document({
        sections: [
          {
            properties: {},
            children: [
              // Title
              new Paragraph({
                children: [
                  new TextRun({
                    text: "DotRepute Chat History",
                    bold: true,
                    size: 36,
                    color: "FF6B35",
                  }),
                ],
                spacing: { after: 200 },
              }),
              // Session info
              new Paragraph({
                children: [
                  new TextRun({
                    text: `Session: ${sessionTitle}`,
                    bold: true,
                    size: 24,
                  }),
                ],
                spacing: { after: 100 },
              }),
              new Paragraph({
                children: [
                  new TextRun({
                    text: `Exported: ${new Date().toLocaleString()}`,
                    size: 20,
                  }),
                ],
                spacing: { after: 100 },
              }),
              new Paragraph({
                children: [
                  new TextRun({
                    text: `Wallet: ${address ? `${address.slice(0, 10)}...${address.slice(-8)}` : "Not connected"}`,
                    size: 20,
                  }),
                ],
                spacing: { after: 100 },
              }),
              new Paragraph({
                children: [
                  new TextRun({
                    text: `Total Messages: ${messages.length}`,
                    size: 20,
                  }),
                ],
                spacing: { after: 400 },
              }),
              // Conversation
              new Paragraph({
                children: [
                  new TextRun({
                    text: "Conversation",
                    bold: true,
                    size: 28,
                    underline: {},
                  }),
                ],
                spacing: { after: 300 },
              }),
              ...messages.flatMap((msg) => [
                new Paragraph({
                  children: [
                    new TextRun({
                      text: `${msg.role === "bot" ? "🤖 Bot" : "👤 You"}`,
                      bold: true,
                      size: 24,
                      color: msg.role === "bot" ? "4A90E2" : "50C878",
                    }),
                    new TextRun({
                      text: ` • ${msg.timestamp.toLocaleString()}`,
                      size: 20,
                      color: "666666",
                    }),
                  ],
                  spacing: { before: 200, after: 100 },
                }),
                new Paragraph({
                  children: [
                    new TextRun({
                      text: msg.content,
                      size: 22,
                    }),
                  ],
                  spacing: { after: 200 },
                }),
                // Add data breakdown if present
                ...(msg.data?.breakdown?.identity &&
                msg.data?.breakdown?.governance &&
                msg.data?.breakdown?.staking &&
                msg.data?.breakdown?.activity
                  ? [
                      new Paragraph({
                        children: [
                          new TextRun({
                            text: `📊 Score Breakdown:`,
                            bold: true,
                            size: 22,
                          }),
                        ],
                        spacing: { after: 100 },
                      }),
                      new Paragraph({
                        children: [
                          new TextRun({
                            text: `Total Score: ${msg.data.totalScore || 0}/${msg.data.maxScore || 100}`,
                            size: 20,
                          }),
                        ],
                      }),
                      new Paragraph({
                        children: [
                          new TextRun({
                            text: `Identity: ${msg.data.breakdown.identity.score}/100`,
                            size: 20,
                          }),
                        ],
                      }),
                      new Paragraph({
                        children: [
                          new TextRun({
                            text: `Governance: ${msg.data.breakdown.governance.score}/100`,
                            size: 20,
                          }),
                        ],
                      }),
                      new Paragraph({
                        children: [
                          new TextRun({
                            text: `Staking: ${msg.data.breakdown.staking.score}/100`,
                            size: 20,
                          }),
                        ],
                      }),
                      new Paragraph({
                        children: [
                          new TextRun({
                            text: `Activity: ${msg.data.breakdown.activity.score}/100`,
                            size: 20,
                          }),
                        ],
                        spacing: { after: 200 },
                      }),
                    ]
                  : msg.data?.totalScore !== undefined
                    ? [
                        new Paragraph({
                          children: [
                            new TextRun({
                              text: `📊 Score: ${msg.data.totalScore}/${msg.data.maxScore || 100}`,
                              bold: true,
                              size: 22,
                            }),
                          ],
                          spacing: { after: 200 },
                        }),
                      ]
                    : []),
              ]),
            ],
          },
        ],
      });

      const blob = await Packer.toBlob(doc);
      const fileName = `dotrepute-${sessionTitle.replace(/[^a-z0-9]/gi, "-").toLowerCase()}-${new Date().getTime()}.docx`;
      saveAs(blob, fileName);
    } catch (error) {
      console.error("Failed to download chat:", error);
      alert("Failed to download chat. Please try again.");
    }
  };

  // Bot response logic - Understanding and responding
  const generateBotResponse = async (userMessage: string): Promise<Message> => {
    const lowerMessage = userMessage.toLowerCase();

    // Check for reputation score query
    if (lowerMessage.includes("reputation") || lowerMessage.includes("score")) {
      // Fetch real data from Polkadot chain if wallet is connected
      if (address && polkadotApi && polkadotApi.isConnected) {
        try {
          const data = await polkadotApi.getReputationScore(address);
          const governance =
            await polkadotApi.getGovernanceParticipation(address);
          const staking = await polkadotApi.getStakingInfo(address);
          const identity = await polkadotApi.getIdentity(address);

          const totalScore = data.totalScore;

          // Calculate badge and percentile
          let badge = "";
          let percentile = "";
          if (totalScore >= 90) {
            badge = "🥇 Elite Contributor";
            percentile = "Top 1%";
          } else if (totalScore >= 80) {
            badge = "🥈 Advanced Contributor";
            percentile = "Top 5%";
          } else if (totalScore >= 70) {
            badge = "🥉 Proficient Contributor";
            percentile = "Top 15%";
          } else if (totalScore >= 60) {
            badge = "🎖️ Competent Contributor";
            percentile = "Top 35%";
          } else if (totalScore >= 50) {
            badge = "⭐ Active Contributor";
            percentile = "Top 50%";
          } else {
            badge = "🌱 Growing Contributor";
            percentile = "Top 65%";
          }

          let reputationContent = `📊 Complete Reputation Analysis\n\n`;
          reputationContent += `${badge}\n`;
          reputationContent += `Overall Score: ${totalScore}/100 (${percentile})\n\n`;

          reputationContent += `━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n`;
          reputationContent += `📈 SCORE BREAKDOWN\n`;
          reputationContent += `━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n`;

          // Identity component (25% weight)
          const identityContribution = Math.floor(
            identity.identityScore * 0.25
          );
          reputationContent += `🆔 Identity: ${identity.identityScore}/100\n`;
          reputationContent += `   Status: ${identity.isVerified ? "✅ Verified" : "⚠️ Not verified"}\n`;
          reputationContent += `   Contribution: ${identityContribution} points (25% weight)\n`;
          reputationContent += `   Impact: ${identity.identityScore >= 80 ? "Excellent ⭐" : identity.identityScore >= 50 ? "Good 👍" : "Needs improvement 📈"}\n\n`;

          // Governance component (25% weight)
          const governanceContribution = Math.floor(
            governance?.participationScore * 0.25
          );
          reputationContent += `🗳️ Governance: ${governance?.participationScore}/100\n`;
          reputationContent += `   Votes Cast: ${governance?.totalVotes} referenda\n`;
          reputationContent += `   Active Votes: ${governance?.activeVotes}\n`;
          reputationContent += `   Contribution: ${governanceContribution} points (25% weight)\n`;
          reputationContent += `   Impact: ${governance.participationScore >= 80 ? "Excellent ⭐" : governance.participationScore >= 50 ? "Good 👍" : "Needs improvement 📈"}\n\n`;

          // Staking component (20% weight)
          const stakingContribution = Math.floor(staking.stakingScore * 0.2);
          const totalStakedDOT = (parseInt(staking.totalStaked) / 1e10).toFixed(
            4
          );
          reputationContent += `💰 Staking: ${staking.stakingScore}/100\n`;
          reputationContent += `   Total Staked: ${totalStakedDOT} DOT\n`;
          reputationContent += `   Contribution: ${stakingContribution} points (20% weight)\n`;
          reputationContent += `   Impact: ${staking.stakingScore >= 80 ? "Excellent ⭐" : staking.stakingScore >= 50 ? "Good 👍" : "Needs improvement 📈"}\n\n`;

          // Activity component (20% weight)
          const activityScore = data.breakdown.activity.score;
          const activityContribution = Math.floor(activityScore * 0.2);
          reputationContent += `🎯 Activity: ${activityScore}/100\n`;
          reputationContent += `   On-chain Engagement: ${activityScore >= 70 ? "High" : activityScore >= 40 ? "Moderate" : "Low"}\n`;
          reputationContent += `   Contribution: ${activityContribution} points (20% weight)\n`;
          reputationContent += `   Impact: ${activityScore >= 80 ? "Excellent ⭐" : activityScore >= 50 ? "Good 👍" : "Needs improvement 📈"}\n\n`;

          // Development component (10% weight - placeholder)
          const devScore = 10;
          const devContribution = Math.floor(devScore * 0.1);
          reputationContent += `💻 Development: ${devScore}/100\n`;
          reputationContent += `   GitHub Activity: Coming soon\n`;
          reputationContent += `   Contribution: ${devContribution} points (10% weight)\n\n`;

          reputationContent += `━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n`;
          reputationContent += `📊 SUMMARY\n`;
          reputationContent += `━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n`;

          const totalContribution =
            identityContribution +
            governanceContribution +
            stakingContribution +
            activityContribution +
            devContribution;
          reputationContent += `Total Weighted Score: ${totalContribution}/100\n\n`;

          // Performance insights
          const allScores = [
            { name: "Identity", score: identity.identityScore },
            { name: "Governance", score: governance.participationScore },
            { name: "Staking", score: staking.stakingScore },
            { name: "Activity", score: activityScore },
          ];
          allScores.sort((a, b) => b.score - a.score);

          reputationContent += `💪 Strongest Area: ${allScores[0].name} (${allScores[0].score}/100)\n`;
          reputationContent += `📈 Focus Area: ${allScores[3].name} (${allScores[3].score}/100)\n\n`;

          // Quick action suggestions
          if (totalScore < 90) {
            reputationContent += `🎯 Quick Wins:\n`;
            if (!identity.isVerified && identity.identity) {
              reputationContent += `• Get registrar verification (+35 points)\n`;
            } else if (!identity.identity) {
              reputationContent += `• Set up on-chain identity (+50 points)\n`;
            }
            if (governance.totalVotes < 10) {
              reputationContent += `• Vote on ${10 - governance.totalVotes} more referenda\n`;
            }
            if (parseInt(staking.totalStaked) === 0) {
              reputationContent += `• Start staking DOT (join nomination pool)\n`;
            }
          }

          return {
            id: Date.now().toString(),
            role: "bot",
            content: reputationContent,
            timestamp: new Date(),
            type: "data",
            data: {
              totalScore: data.totalScore,
              maxScore: data.maxScore,
              badge,
              percentile,
              breakdown: data.breakdown,
              details: data.details,
              contributions: {
                identity: identityContribution,
                governance: governanceContribution,
                staking: stakingContribution,
                activity: activityContribution,
                development: devContribution,
              },
            },
          };
        } catch (error) {
          console.error("Failed to fetch reputation:", error);
          return {
            id: Date.now().toString(),
            role: "bot",
            content:
              "Unable to fetch your reputation score from the blockchain. Please try again later.",
            timestamp: new Date(),
            type: "text",
          };
        }
      }

      // Not connected
      return {
        id: Date.now().toString(),
        role: "bot",
        content:
          address && !polkadotApi?.isConnected
            ? "Connecting to Polkadot network... Please try again in a moment."
            : "Please connect your wallet to view your reputation score.",
        timestamp: new Date(),
        type: "text",
      };
    }

    // Check for identity query (general, not analysis)
    if (
      lowerMessage.includes("identity") &&
      !lowerMessage.includes("analyze") &&
      !lowerMessage.includes("verification")
    ) {
      if (address && polkadotApi && polkadotApi.isConnected) {
        try {
          const identity = await polkadotApi.getIdentity(address);

          if (!identity.identity) {
            return {
              id: Date.now().toString(),
              role: "bot",
              content:
                "⚠️ No On-Chain Identity Found\n\nYou don't have an on-chain identity set up yet.\n\nBenefits of setting up identity:\n• Increases trust and reputation (+25 points potential)\n• Helps others recognize you in the ecosystem\n• Required for many governance roles\n• Shows commitment to transparency\n\nHow to set up:\n1. Visit Polkadot.js Apps (apps.polkadot.io)\n2. Go to Accounts → Set on-chain identity\n3. Fill in your information\n4. Request judgement from a registrar\n\nWould you like detailed instructions?",
              timestamp: new Date(),
              type: "text",
            };
          }

          const identityData = identity.identity as any;
          const info = identityData.info || {};

          let identityContent = `🆔 Identity Score: ${identity.identityScore}/100\n\n`;
          identityContent += `Status: ${identity.isVerified ? "✅ Verified by registrar" : "⚠️ Not yet verified"}\n\n`;

          identityContent += `Identity Fields:\n`;
          identityContent += `• Display Name: ${info.display?.Raw ? `✓ ${Buffer.from(info.display.Raw.slice(2), "hex").toString()}` : "✗ Not set"}\n`;
          identityContent += `• Legal Name: ${info.legal?.Raw ? "✓ Set" : "✗ Not set"}\n`;
          identityContent += `• Email: ${info.email?.Raw ? "✓ Set" : "✗ Not set"}\n`;
          identityContent += `• Twitter: ${info.twitter?.Raw ? `✓ @${Buffer.from(info.twitter.Raw.slice(2), "hex").toString()}` : "✗ Not set"}\n`;
          identityContent += `• Web: ${info.web?.Raw ? "✓ Set" : "✗ Not set"}\n`;
          identityContent += `• Riot/Matrix: ${info.riot?.Raw ? "✓ Set" : "✗ Not set"}\n\n`;

          if (identityData.judgements && identityData.judgements.length > 0) {
            identityContent += `Registrar Judgements:\n`;
            identityData.judgements.forEach((judgement: any, idx: number) => {
              identityContent += `• Registrar ${idx + 1}: ${judgement[1]}\n`;
            });
            identityContent += `\n`;
          }

          identityContent += `Impact on Reputation:\n`;
          identityContent += `• Identity contributes 25% to your total score\n`;
          identityContent += `• Current contribution: ${Math.floor(identity.identityScore * 0.25)} points\n`;

          if (identity.identityScore < 100) {
            identityContent += `\n💡 To improve: ${!identity.isVerified ? "Get registrar verification" : "Add more identity fields"}`;
          }

          return {
            id: Date.now().toString(),
            role: "bot",
            content: identityContent,
            timestamp: new Date(),
            type: "data",
            data: {
              identityScore: identity.identityScore,
              isVerified: identity.isVerified,
              fields: info,
            },
          };
        } catch (error) {
          console.error("Failed to fetch identity data:", error);
          return {
            id: Date.now().toString(),
            role: "bot",
            content:
              "Unable to fetch your identity data from the blockchain. Please try again later.",
            timestamp: new Date(),
            type: "text",
          };
        }
      }

      return {
        id: Date.now().toString(),
        role: "bot",
        content:
          address && !polkadotApi?.isConnected
            ? "Connecting to Polkadot network... Please try again in a moment."
            : "Please connect your wallet to view your identity information.",
        timestamp: new Date(),
        type: "text",
      };
    }

    // Check for governance query
    const hasGovernanceKeyword =
      lowerMessage.includes("governance") || lowerMessage.includes("vote");
    const extracted = extractPolkadotAddress(userMessage);
    const targetAddress = extracted?.address || address;
    if (hasGovernanceKeyword) {
      if (!targetAddress) {
        return {
          id: Date.now().toString(),
          role: "bot",
          content: extracted
            ? "Invalid address. Please enter a valid Polkadot or Kusama address."
            : "Please connect your wallet or enter a valid address.",
          timestamp: new Date(),
          type: "text",
        };
      }

      try {
        const apiUrl = `/api/subscan?address=${encodeURIComponent(targetAddress)}`;
        const response = await fetch(apiUrl);

        if (!response.ok) throw new Error("API error");

        const result = await response.json();
        const governanceData = result?.data;

        if (!governanceData || typeof governanceData.count !== "number") {
          throw new Error("Invalid response");
        }
        const governance = governanceData;
        const totalVotes = governance?.count;
        const referendaList = governance?.referenda || [];
        const participationScore = Math.min(
          100,
          Math.floor(totalVotes * (100 / 30))
        );

        // Calculate contribution to overall reputation
        const governanceContribution = participationScore;

        let governanceContent = `Your governance participation score is ${participationScore}.\n\n`;
        // Score overview
        if (totalVotes > 0) {
          governanceContent += `Recent Voting Activity:\n`;
          referendaList.slice(0, 5).forEach((item: any) => {
            governanceContent += `• ${item}\n`;
          });
        } else {
          governanceContent += `No recent voting activity found.\n`;
        }

        governanceContent += `\n`;

        if (participationScore >= 80) {
          governanceContent += `You're doing fantastic! Your governance participation is exemplary.`;
        } else if (participationScore >= 60) {
          governanceContent += `Great work! You're an active governance participant.`;
        } else if (participationScore >= 40) {
          governanceContent += `Good progress! Increase your voting to boost your score.`;
        } else {
          governanceContent += `Ready to make an impact? Start voting to improve your score!`;
        }

        return {
          id: Date.now().toString(),
          role: "bot",
          content: governanceContent,
          timestamp: new Date(),
          type: "data",
          data: {
            participationScore,
            totalVotes,
            governanceContribution,
          },
        };
      } catch (error) {
        console.error("Failed to fetch governance data:", error);
        return {
          id: Date.now().toString(),
          role: "bot",
          content:
            "Unable to fetch your governance data from the blockchain. Please try again later.",
          timestamp: new Date(),
          type: "text",
        };
      }
    }

    // Check for staking query
    if (
      lowerMessage.includes("staking") ||
      lowerMessage.includes("stake") ||
      lowerMessage.includes("point")
    ) {
      if (!targetAddress) {
        return {
          id: Date.now().toString(),
          role: "bot",
          content: extracted
            ? "Invalid address. Please enter a valid Polkadot/Kusama address."
            : "Please connect your wallet or enter a valid address.",
          timestamp: new Date(),
          type: "text",
        };
      }

      try {
        const res = await fetch(
          `/api/staking?address=${encodeURIComponent(targetAddress)}`
        );

        if (!res.ok) throw new Error("API error");

        const result = await res.json();
        const pointsData = result?.data || [];

        if (!Array.isArray(pointsData) || pointsData.length === 0) {
          return {
            id: Date.now().toString(),
            role: "bot",
            content: `No staking points found for this address.`,
            timestamp: new Date(),
            type: "text",
          };
        }

        const totalPoints = pointsData.reduce(
          (sum: number, e: any) => sum + (e.rewardPoints || 0),
          0
        );
        const avgPoints = Math.round(totalPoints / pointsData.length);
        const erasCount = pointsData.length;

        const stakingScore = Math.min(100, Math.round(avgPoints / 1000)); // ~100k points → 100

        let content = `Staking Points History\n\n`;

        content += `━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n`;
        content += `POINT SUMMARY\n`;
        content += `━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n`;

        content += `Staking Score: ${stakingScore}/100\n`;
        content += `Rating: ${
          stakingScore >= 80
            ? "Excellent"
            : stakingScore >= 60
              ? "Good"
              : stakingScore >= 40
                ? "Moderate"
                : "Growing"
        }\n`;
        content += `Eras with Points: ${erasCount}\n`;
        content += `Average Points/Era: ${avgPoints.toLocaleString()}\n`;
        content += `Total Points: ${totalPoints.toLocaleString()}\n\n`;

        content += `━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n`;
        content += `RECENT ERAS\n`;
        content += `━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n`;

        pointsData.slice(0, 8).forEach((era: any) => {
          const points = era.rewardPoints || 0;
          const status = points > 0 ? "Active" : "No points";
          content += `• Era ${era.era}: ${points.toLocaleString()} pts ${status}\n`;
        });

        if (pointsData.length > 8) {
          content += `... and ${pointsData.length - 8} more eras\n`;
        }

        content += `\n`;

        if (stakingScore >= 80) {
          content += `Outstanding validator/nominator performance!`;
        } else if (stakingScore >= 60) {
          content += `Great consistency! Keep up the good work.`;
        } else if (stakingScore >= 40) {
          content += `Good start. Aim for more consistent points.`;
        } else {
          content += `Start staking or nominate active validators to earn points!`;
        }

        return {
          id: Date.now().toString(),
          role: "bot",
          content,
          timestamp: new Date(),
          type: "data",
          data: {
            stakingScore,
            avgPoints,
            totalPoints,
            erasCount,
            recentEras: pointsData.slice(0, 8),
            address: targetAddress,
          },
        };
      } catch (error) {
        console.error("Staking points error:", error);
        return {
          id: Date.now().toString(),
          role: "bot",
          content: "Unable to load staking points. Please try again later.",
          timestamp: new Date(),
          type: "text",
        };
      }
    }

    // Check for activity query
    if (
      lowerMessage.includes("activity") ||
      lowerMessage.includes("contribution")
    ) {
      return {
        id: Date.now().toString(),
        role: "bot",
        content:
          "Your activity score is 71/100. You've made 24 on-chain transactions this month.\n\nRecent Activities:\n• Voted on Referendum #245 (2 hours ago)\n• Increased Stake by 100 DOT (1 day ago)\n• GitHub Contribution - Merged PR to polkadot-sdk (3 days ago)\n• Identity Update - Added Twitter handle (1 week ago)\n• Treasury Proposal Discussion (2 weeks ago)\n\nYour consistent engagement across governance, staking, and development shows strong ecosystem participation.",
        timestamp: new Date(),
        type: "text",
      };
    }

    // Check for identity analysis
    if (
      lowerMessage.includes("identity") &&
      (lowerMessage.includes("analyze") ||
        lowerMessage.includes("verification"))
    ) {
      if (address && polkadotApi && polkadotApi.isConnected) {
        try {
          const identity = await polkadotApi.getIdentity(address);

          return {
            id: Date.now().toString(),
            role: "bot",
            content: identity.identity
              ? `Your identity verification score is ${identity.identityScore}/100.\n\n${identity.isVerified ? "✅ Verified by registrar" : "⚠️ Not yet verified"}\n\nTo improve your identity score:\n• ${identity.isVerified ? "Maintain" : "Get"} registrar verification\n• Add more identity fields (display name, email, Twitter, etc.)\n• Keep your information up to date\n\nA strong identity score significantly boosts your overall reputation!`
              : "You don't have an on-chain identity set yet.\n\nSetting up your identity:\n1. Go to Polkadot.js Apps\n2. Navigate to Accounts > Set on-chain identity\n3. Fill in your information\n4. Request judgement from a registrar\n\nThis will significantly boost your reputation score!",
            timestamp: new Date(),
            type: "text",
          };
        } catch (error) {
          console.error("Failed to analyze identity:", error);
        }
      }

      return {
        id: Date.now().toString(),
        role: "bot",
        content:
          "Please connect your wallet to analyze your identity verification.",
        timestamp: new Date(),
        type: "text",
      };
    }

    // Check for leaderboard comparison
    if (
      lowerMessage.includes("leaderboard") ||
      lowerMessage.includes("compare") ||
      lowerMessage.includes("ranking")
    ) {
      if (address && polkadotApi && polkadotApi.isConnected) {
        try {
          const data = await polkadotApi.getReputationScore(address);
          const governance =
            await polkadotApi.getGovernanceParticipation(address);
          const staking = await polkadotApi.getStakingInfo(address);
          const identity = await polkadotApi.getIdentity(address);

          const totalScore = data.totalScore;

          // Calculate estimated rank based on score (higher score = better rank)
          // Percentile calculation: Score ranges mapped to percentiles
          let estimatedRank = 0;
          let percentile = "";
          let badge = "";

          if (totalScore >= 90) {
            estimatedRank = Math.floor(Math.random() * 50) + 1; // Top 50
            percentile = "Top 1%";
            badge = "🥇 Elite";
          } else if (totalScore >= 80) {
            estimatedRank = Math.floor(Math.random() * 200) + 51; // 51-250
            percentile = "Top 5%";
            badge = "🥈 Advanced";
          } else if (totalScore >= 70) {
            estimatedRank = Math.floor(Math.random() * 500) + 251; // 251-750
            percentile = "Top 15%";
            badge = "🥉 Proficient";
          } else if (totalScore >= 60) {
            estimatedRank = Math.floor(Math.random() * 1000) + 751; // 751-1750
            percentile = "Top 35%";
            badge = "🎖️ Competent";
          } else if (totalScore >= 50) {
            estimatedRank = Math.floor(Math.random() * 1500) + 1751; // 1751-3250
            percentile = "Top 50%";
            badge = "⭐ Active";
          } else {
            estimatedRank = Math.floor(Math.random() * 2000) + 3251; // 3251+
            percentile = "Top 65%";
            badge = "🌱 Growing";
          }

          let leaderboardContent = `🏆 Leaderboard Analysis\n\n`;
          leaderboardContent += `${badge} Badge\n`;
          leaderboardContent += `Your Score: ${totalScore}/100\n`;
          leaderboardContent += `Estimated Rank: #${estimatedRank}\n`;
          leaderboardContent += `Percentile: ${percentile}\n\n`;

          leaderboardContent += `Score Breakdown:\n`;
          leaderboardContent += `• Identity: ${identity.identityScore}/100 ${identity.isVerified ? "✅" : "⚠️"}\n`;
          leaderboardContent += `• Governance: ${governance.participationScore}/100 (${governance.totalVotes} votes)\n`;
          leaderboardContent += `• Staking: ${staking.stakingScore}/100\n`;
          leaderboardContent += `• Activity: ${data.breakdown.activity.score}/100\n\n`;

          // Simulated top performers for context
          leaderboardContent += `📊 Top Contributors Context:\n`;
          leaderboardContent += `1. 15kPFy... - 98/100 (Elite contributor)\n`;
          leaderboardContent += `2. 14jQRp... - 96/100 (Verified identity, high governance)\n`;
          leaderboardContent += `3. 13mNPw... - 94/100 (Heavy staker, active voter)\n`;
          leaderboardContent += `...\n`;
          leaderboardContent += `${estimatedRank}. You - ${totalScore}/100\n\n`;

          // Identify strengths and weaknesses
          const scores = [
            { name: "Identity", score: identity.identityScore },
            { name: "Governance", score: governance.participationScore },
            { name: "Staking", score: staking.stakingScore },
            { name: "Activity", score: data.breakdown.activity.score },
          ];

          scores.sort((a, b) => b.score - a.score);
          const strongest = scores[0];
          const weakest = scores[scores.length - 1];

          leaderboardContent += `💪 Your Strength: ${strongest.name} (${strongest.score}/100)\n`;
          leaderboardContent += `📈 Improvement Area: ${weakest.name} (${weakest.score}/100)\n\n`;

          // Provide ranking improvement tips
          const scoreGap = totalScore < 90 ? 90 - totalScore : 0;
          if (scoreGap > 0) {
            leaderboardContent += `🎯 To reach Elite tier (90+):\n`;
            leaderboardContent += `• Gain ${scoreGap} more points\n`;

            if (weakest.score < 70) {
              leaderboardContent += `• Focus on improving ${weakest.name}\n`;
            }
            if (!identity.isVerified) {
              leaderboardContent += `• Get identity verified (+35 points potential)\n`;
            }
            if (governance.totalVotes < 20) {
              leaderboardContent += `• Vote on more referenda (target: 20+ votes)\n`;
            }
            if (staking.stakingScore < 50) {
              leaderboardContent += `• Increase staking participation\n`;
            }
          } else {
            leaderboardContent += `🎉 You're in the Elite tier! Keep up the excellent work!`;
          }

          return {
            id: Date.now().toString(),
            role: "bot",
            content: leaderboardContent,
            timestamp: new Date(),
            type: "data",
            data: {
              totalScore,
              rank: estimatedRank,
              percentile,
              badge,
              breakdown: data.breakdown,
              strongest: strongest.name,
              weakest: weakest.name,
            },
          };
        } catch (error) {
          console.error("Failed to fetch leaderboard:", error);
          return {
            id: Date.now().toString(),
            role: "bot",
            content:
              "Unable to fetch leaderboard data from the blockchain. Please try again later.",
            timestamp: new Date(),
            type: "text",
          };
        }
      }

      return {
        id: Date.now().toString(),
        role: "bot",
        content:
          address && !polkadotApi?.isConnected
            ? "Connecting to Polkadot network... Please try again in a moment."
            : "Please connect your wallet to view leaderboard rankings.",
        timestamp: new Date(),
        type: "text",
      };
    }

    // Check for historical trends
    if (
      lowerMessage.includes("historical") ||
      lowerMessage.includes("trends") ||
      lowerMessage.includes("history")
    ) {
      if (address && polkadotApi && polkadotApi.isConnected) {
        try {
          const currentData = await polkadotApi.getReputationScore(address);
          const governance =
            await polkadotApi.getGovernanceParticipation(address);
          const staking = await polkadotApi.getStakingInfo(address);
          const identity = await polkadotApi.getIdentity(address);

          // Calculate trend estimates based on current scores
          const totalScore = currentData.totalScore;
          const trend30d = Math.floor(totalScore * 0.15); // Estimate 15% growth
          const trend90d = Math.floor(totalScore * 0.35); // Estimate 35% growth

          const score30dAgo = Math.max(0, totalScore - trend30d);
          const score90dAgo = Math.max(0, totalScore - trend90d);

          let trendContent = `📈 Historical Reputation Trends\n\n`;
          trendContent += `Current Score: ${totalScore}/100\n\n`;
          trendContent += `30 Day Trend: ${trend30d > 0 ? "⬆️ +" : ""}${trend30d} points\n`;
          trendContent += `90 Day Trend: ${trend90d > 0 ? "⬆️ +" : ""}${trend90d} points\n\n`;

          trendContent += `Score Timeline:\n`;
          trendContent += `• Today: ${totalScore}/100\n`;
          trendContent += `• 1 month ago: ~${score30dAgo}/100\n`;
          trendContent += `• 3 months ago: ~${score90dAgo}/100\n\n`;

          trendContent += `Component Breakdown:\n`;
          trendContent += `• Identity: ${identity.identityScore}/100 ${identity.isVerified ? "✅" : "⚠️"}\n`;
          trendContent += `• Governance: ${governance.participationScore}/100 (${governance.totalVotes} votes)\n`;
          trendContent += `• Staking: ${staking.stakingScore}/100\n`;
          trendContent += `• Activity: ${currentData.breakdown.activity.score}/100\n\n`;

          // Provide insights
          const topScore = Math.max(
            identity.identityScore,
            governance.participationScore,
            staking.stakingScore,
            currentData.breakdown.activity.score
          );

          if (topScore === identity.identityScore) {
            trendContent += `💪 Strongest area: Identity verification\n`;
          } else if (topScore === governance.participationScore) {
            trendContent += `💪 Strongest area: Governance participation\n`;
          } else if (topScore === staking.stakingScore) {
            trendContent += `💪 Strongest area: Staking commitment\n`;
          } else {
            trendContent += `💪 Strongest area: On-chain activity\n`;
          }

          trendContent += `\n${trend30d > 5 ? "🎉 Excellent growth trajectory!" : "📊 Steady progress - keep going!"}`;

          return {
            id: Date.now().toString(),
            role: "bot",
            content: trendContent,
            timestamp: new Date(),
            type: "data",
            data: {
              totalScore,
              trend30d,
              trend90d,
              breakdown: currentData.breakdown,
            },
          };
        } catch (error) {
          console.error("Failed to fetch historical trends:", error);
          return {
            id: Date.now().toString(),
            role: "bot",
            content:
              "Unable to fetch historical trends from the blockchain. Please try again later.",
            timestamp: new Date(),
            type: "text",
          };
        }
      }

      return {
        id: Date.now().toString(),
        role: "bot",
        content:
          address && !polkadotApi?.isConnected
            ? "Connecting to Polkadot network... Please try again in a moment."
            : "Please connect your wallet to view historical trends.",
        timestamp: new Date(),
        type: "text",
      };
    }

    // Check for recommendations
    if (
      lowerMessage.includes("recommend") ||
      lowerMessage.includes("suggestion") ||
      lowerMessage.includes("improve")
    ) {
      if (address && polkadotApi && polkadotApi.isConnected) {
        try {
          const data = await polkadotApi.getReputationScore(address);
          const governance =
            await polkadotApi.getGovernanceParticipation(address);
          const staking = await polkadotApi.getStakingInfo(address);
          const identity = await polkadotApi.getIdentity(address);

          const governanceScore = data.breakdown.governance.score;
          const stakingScore = data.breakdown.staking.score;
          const identityScore = data.breakdown.identity.score;
          const activityScore = data.breakdown.activity.score;

          let recommendations = "💡 Personalized Recommendations\n\n";
          recommendations += `Current Score: ${data.totalScore}/100\n\n`;
          recommendations += `Here's how to maximize your reputation:\n\n`;

          let totalPotentialGain = 0;
          let recCount = 1;

          // Identity recommendations
          if (identityScore < 50) {
            const gain = Math.floor((100 - identityScore) * 0.25); // 25% weight
            totalPotentialGain += gain;
            recommendations += `${recCount}. 🆔 HIGH PRIORITY: Set up on-chain identity\n`;
            recommendations += `   Current: ${identityScore}/100 | Potential: +${gain} points\n`;
            recommendations += `   Steps:\n`;
            recommendations += `   • Go to Polkadot.js Apps → Accounts\n`;
            recommendations += `   • Set identity (display name, email, etc.)\n`;
            recommendations += `   • Request registrar verification\n`;
            recommendations += `   Time: 10-15 minutes\n\n`;
            recCount++;
          } else if (identityScore < 100 && !identity.isVerified) {
            const gain = Math.floor((100 - identityScore) * 0.25);
            totalPotentialGain += gain;
            recommendations += `${recCount}. ✅ Get registrar verification for your identity\n`;
            recommendations += `   Current: ${identityScore}/100 | Potential: +${gain} points\n`;
            recommendations += `   Your identity is set but not verified by a registrar\n\n`;
            recCount++;
          }

          // Governance recommendations
          if (governanceScore < 70) {
            const votesNeeded = Math.ceil((70 - governanceScore) / 5); // Rough estimate
            const gain = Math.floor((70 - governanceScore) * 0.25); // 25% weight
            totalPotentialGain += gain;
            recommendations += `${recCount}. 🗳️ ${governanceScore < 30 ? "HIGH PRIORITY: " : ""}Increase governance participation\n`;
            recommendations += `   Current: ${governanceScore}/100 (${governance.totalVotes} votes) | Potential: +${gain} points\n`;
            recommendations += `   Action: Vote on ~${votesNeeded} more referenda\n`;
            recommendations += `   Check: Polkassembly or Subsquare for active proposals\n`;
            recommendations += `   Time: 5-10 minutes per vote\n\n`;
            recCount++;
          }

          // Staking recommendations
          if (stakingScore < 70) {
            const currentStaked = parseFloat(
              (parseInt(staking.totalStaked) / 1e10).toFixed(4)
            );
            const gain = Math.floor((70 - stakingScore) * 0.2); // 20% weight
            totalPotentialGain += gain;
            recommendations += `${recCount}. 💰 ${stakingScore < 30 ? "HIGH PRIORITY: " : ""}Increase staking commitment\n`;
            recommendations += `   Current: ${stakingScore}/100 (${currentStaked} DOT) | Potential: +${gain} points\n`;
            recommendations += `   Options:\n`;
            if (currentStaked === 0) {
              recommendations += `   • Join a nomination pool (minimum 1 DOT)\n`;
              recommendations += `   • Nominate validators (minimum 250 DOT)\n`;
            } else {
              recommendations += `   • Increase your stake for higher score\n`;
              recommendations += `   • Ensure your validators are active\n`;
            }
            recommendations += `   Benefit: Passive staking rewards + reputation boost\n\n`;
            recCount++;
          }

          // Activity recommendations
          if (activityScore < 70) {
            const gain = Math.floor((70 - activityScore) * 0.2); // 20% weight
            totalPotentialGain += gain;
            recommendations += `${recCount}. 🎯 Increase on-chain activity\n`;
            recommendations += `   Current: ${activityScore}/100 | Potential: +${gain} points\n`;
            recommendations += `   Ideas:\n`;
            recommendations += `   • Participate in treasury proposals\n`;
            recommendations += `   • Engage in governance discussions\n`;
            recommendations += `   • Make regular on-chain transactions\n`;
            recommendations += `   • Join community initiatives\n\n`;
            recCount++;
          }

          // Summary
          if (totalPotentialGain > 0) {
            recommendations += `\n🎯 Potential Total Gain: +${totalPotentialGain} points\n`;
            recommendations += `📊 Projected Score: ${Math.min(100, data.totalScore + totalPotentialGain)}/100\n\n`;

            if (recCount === 2) {
              recommendations += `Focus on completing this recommendation to see significant improvement!`;
            } else {
              recommendations += `Start with the highest priority items for maximum impact!`;
            }
          } else {
            recommendations += `🎉 Excellent work! Your scores are already strong.\n\n`;
            recommendations += `Maintenance tips:\n`;
            recommendations += `• Continue voting on governance proposals\n`;
            recommendations += `• Keep your staking active\n`;
            recommendations += `• Maintain your on-chain identity\n`;
            recommendations += `• Stay active in the ecosystem`;
          }

          return {
            id: Date.now().toString(),
            role: "bot",
            content: recommendations,
            timestamp: new Date(),
            type: "data",
            data: {
              totalScore: data.totalScore,
              potentialGain: totalPotentialGain,
              projectedScore: Math.min(
                100,
                data.totalScore + totalPotentialGain
              ),
              breakdown: data.breakdown,
            },
          };
        } catch (error) {
          console.error("Failed to generate recommendations:", error);
          return {
            id: Date.now().toString(),
            role: "bot",
            content:
              "Unable to generate personalized recommendations. Please try again later.",
            timestamp: new Date(),
            type: "text",
          };
        }
      }

      return {
        id: Date.now().toString(),
        role: "bot",
        content:
          address && !polkadotApi?.isConnected
            ? "Connecting to Polkadot network... Please try again in a moment."
            : "Please connect your wallet to get personalized recommendations.",
        timestamp: new Date(),
        type: "text",
      };
    }

    // Check for help or what can you do
    if (lowerMessage.includes("help") || lowerMessage.includes("what can")) {
      return {
        id: Date.now().toString(),
        role: "bot",
        content:
          "I can help you with:\n\n• Check your reputation score\n• Analyze identity verification\n• View governance participation\n• Track staking activity\n• Monitor recent contributions\n• Compare with leaderboard\n• View historical trends\n• Get personalized recommendations\n\nJust ask me anything about your reputation! You can also:\n• Bookmark important messages\n• Download chat history (Word document)\n• Start new chat sessions",
        timestamp: new Date(),
        type: "text",
      };
    }

    // Default response - ask for clarification
    return {
      id: Date.now().toString(),
      role: "bot",
      content:
        "I'm not sure I understood that. Could you please clarify? You can ask me about your reputation score, identity, governance, staking, or activity.",
      timestamp: new Date(),
      type: "question",
    };
  };

  // Handle sending message
  const handleSendMessage = async () => {
    if (!inputValue.trim()) return;

    // Add user message
    const userMessage: Message = {
      id: Date.now().toString(),
      role: "user",
      content: inputValue,
      timestamp: new Date(),
      type: "text",
    };

    const updatedMessages = [...messages, userMessage];
    setMessages(updatedMessages);

    // Update session messages
    setSessionMessages((prev) => ({
      ...prev,
      [currentSessionId]: updatedMessages,
    }));

    // Update chat session
    setChatSessions((prev) =>
      prev.map((session) =>
        session.id === currentSessionId
          ? {
              ...session,
              lastMessage:
                inputValue.substring(0, 50) +
                (inputValue.length > 50 ? "..." : ""),
              timestamp: new Date(),
              messageCount: session.messageCount + 1,
              title:
                session.messageCount === 0
                  ? inputValue.substring(0, 30) +
                    (inputValue.length > 30 ? "..." : "")
                  : session.title,
            }
          : session
      )
    );

    setInputValue("");
    setIsTyping(true);

    // Simulate bot thinking time
    await new Promise((resolve) => setTimeout(resolve, 1000));

    // Generate bot response
    const botResponse = await generateBotResponse(userMessage.content);
    const finalMessages = [...updatedMessages, botResponse];
    setMessages(finalMessages);

    // Save bot response to session messages
    setSessionMessages((prev) => ({
      ...prev,
      [currentSessionId]: finalMessages,
    }));

    setIsTyping(false);
  };

  // Handle Enter key press
  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

  // Prevent flash of unstyled content
  if (!mounted) return null;

  return (
    <div
      className={`relative min-h-screen transition-colors duration-300 ${
        theme === "light" ? "bg-white text-black" : "bg-black text-white"
      }`}
    >
      {/* Venture Navbar */}
      <VentureNavbar
        theme={theme}
        onToggleTheme={toggleTheme}
        currentPath="/dashboard"
        isLoggedIn={isLoggedIn}
        onLogout={handleLogout}
      />

      {/* Main Dashboard Layout */}
      <div className="pt-24 h-screen flex flex-col">
        <div className="flex-1 flex overflow-hidden">
          {/* Left Sidebar - Chat History & Bookmarks */}
          <aside
            className={`w-80 border-r flex flex-col transition-colors ${
              theme === "light"
                ? "border-black/10 bg-gray-50"
                : "border-white/5 bg-black"
            }`}
          >
            {/* Sidebar Header - Login/Signup or User Info */}
            <div
              className={`p-4 border-b ${
                theme === "light" ? "border-black/10" : "border-white/5"
              }`}
            >
              {!isLoggedIn ? (
                <div className="space-y-3">
                  <p
                    className={`text-sm ${
                      theme === "light" ? "text-gray-600" : "text-gray-400"
                    }`}
                  >
                    Sign in to save your chat history
                  </p>
                  <div className="flex gap-2">
                    <Link
                      href="/auth"
                      className={`flex-1 border px-3 py-2 text-xs uppercase tracking-wider font-medium transition-colors text-center ${
                        theme === "light"
                          ? "border-black/20 hover:bg-black/5 text-gray-700"
                          : "border-white/10 hover:bg-white/5 text-gray-400"
                      }`}
                    >
                      <LogIn className="w-4 h-4 inline mr-2" />
                      Login
                    </Link>
                    <Link
                      href="/auth"
                      className={`flex-1 border px-3 py-2 text-xs uppercase tracking-wider font-medium transition-colors text-center ${
                        theme === "light"
                          ? "border-black/20 bg-black text-white hover:bg-black/90"
                          : "border-white/10 bg-white text-black hover:bg-white/90"
                      }`}
                    >
                      <UserPlus className="w-4 h-4 inline mr-2" />
                      Sign Up
                    </Link>
                  </div>
                </div>
              ) : (
                <div className="flex items-center gap-3">
                  <div
                    className={`border p-2 ${
                      theme === "light"
                        ? "border-black/20 bg-white"
                        : "border-white/10 bg-black/40"
                    }`}
                  >
                    <User className="w-5 h-5" />
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="font-medium truncate">
                      {address ? `Wallet` : "User Account"}
                    </div>
                    <div
                      className={`text-xs font-mono truncate ${
                        theme === "light" ? "text-gray-600" : "text-gray-500"
                      }`}
                    >
                      {address
                        ? `${address.slice(0, 6)}...${address.slice(-4)}`
                        : "Not connected"}
                    </div>
                  </div>
                  {address && (
                    <button
                      onClick={downloadChatAsWord}
                      className={`border p-2 transition-colors ${
                        theme === "light"
                          ? "border-black/20 hover:bg-black/5"
                          : "border-white/10 hover:bg-white/5"
                      }`}
                      title="Download chat as Word document"
                    >
                      <Download className="w-4 h-4" />
                    </button>
                  )}
                </div>
              )}
            </div>

            {/* Tab Switcher */}
            <div
              className={`flex border-b ${
                theme === "light" ? "border-black/10" : "border-white/5"
              }`}
            >
              <button
                onClick={() => setSidebarTab("chats")}
                className={`flex-1 px-4 py-3 text-xs uppercase tracking-wider font-medium transition-colors ${
                  sidebarTab === "chats"
                    ? theme === "light"
                      ? "border-b-2 border-black bg-white text-black"
                      : "border-b-2 border-white bg-black text-white"
                    : theme === "light"
                      ? "text-gray-600 hover:bg-gray-100"
                      : "text-gray-500 hover:bg-white/5"
                }`}
              >
                <MessageSquare className="w-4 h-4 inline mr-2" />
                Chats
              </button>
              <button
                onClick={() => setSidebarTab("bookmarks")}
                className={`flex-1 px-4 py-3 text-xs uppercase tracking-wider font-medium transition-colors ${
                  sidebarTab === "bookmarks"
                    ? theme === "light"
                      ? "border-b-2 border-black bg-white text-black"
                      : "border-b-2 border-white bg-black text-white"
                    : theme === "light"
                      ? "text-gray-600 hover:bg-gray-100"
                      : "text-gray-500 hover:bg-white/5"
                }`}
              >
                <Bookmark className="w-4 h-4 inline mr-2" />
                Saved ({bookmarkedMessages.length})
              </button>
            </div>

            {/* Sidebar Content */}
            <div className="flex-1 overflow-y-auto">
              {sidebarTab === "chats" ? (
                <div className="p-3 space-y-2">
                  {/* New Chat Button */}
                  <button
                    onClick={createNewChat}
                    className={`w-full border p-3 text-sm transition-colors flex items-center gap-3 ${
                      theme === "light"
                        ? "border-black/20 hover:bg-white hover:border-black/30"
                        : "border-white/10 hover:bg-white/5 hover:border-white/20"
                    }`}
                  >
                    <Plus className="w-4 h-4" />
                    <span className="font-medium">New Chat</span>
                  </button>

                  {/* Chat Sessions List */}
                  {chatSessions.map((session) => (
                    <button
                      key={session.id}
                      onClick={() => switchSession(session.id)}
                      className={`w-full border p-3 text-left transition-colors ${
                        session.id === currentSessionId
                          ? theme === "light"
                            ? "border-black/30 bg-orange-50"
                            : "border-orange-900/30 bg-orange-950/20"
                          : theme === "light"
                            ? "border-black/10 hover:bg-white hover:border-black/20"
                            : "border-white/5 hover:bg-white/5 hover:border-white/10"
                      }`}
                    >
                      <div className="flex items-start justify-between gap-2">
                        <div className="flex-1 min-w-0">
                          <div className="font-medium text-sm truncate mb-1">
                            {session.title}
                          </div>
                          <div
                            className={`text-xs truncate ${
                              theme === "light"
                                ? "text-gray-600"
                                : "text-gray-500"
                            }`}
                          >
                            {session.lastMessage || "No messages yet"}
                          </div>
                          <div
                            className={`text-xs font-mono mt-1 ${
                              theme === "light"
                                ? "text-gray-500"
                                : "text-gray-600"
                            }`}
                          >
                            {session.timestamp.toLocaleDateString()}
                          </div>
                        </div>
                        <div
                          className={`text-xs font-mono ${
                            theme === "light"
                              ? "text-gray-500"
                              : "text-gray-600"
                          }`}
                        >
                          {session.messageCount}
                        </div>
                      </div>
                    </button>
                  ))}
                </div>
              ) : (
                <div className="p-3 space-y-2">
                  {bookmarkedMessages.length === 0 ? (
                    <div
                      className={`text-center py-8 text-sm ${
                        theme === "light" ? "text-gray-600" : "text-gray-500"
                      }`}
                    >
                      <Star className="w-8 h-8 mx-auto mb-2 opacity-50" />
                      No bookmarked messages yet
                    </div>
                  ) : (
                    bookmarkedMessages.map((msg) => (
                      <div
                        key={msg.id}
                        className={`border p-3 text-sm ${
                          theme === "light"
                            ? "border-black/10 bg-white"
                            : "border-white/5 bg-black/20"
                        }`}
                      >
                        <div className="flex items-start gap-2 mb-2">
                          <div
                            className={
                              theme === "light"
                                ? "text-gray-600"
                                : "text-gray-500"
                            }
                          >
                            {msg.role === "bot" ? (
                              <Bot className="w-4 h-4" />
                            ) : (
                              <User className="w-4 h-4" />
                            )}
                          </div>
                          <div className="flex-1 min-w-0">
                            <p className="line-clamp-3">{msg.content}</p>
                          </div>
                        </div>
                        <div
                          className={`text-xs font-mono ${
                            theme === "light"
                              ? "text-gray-500"
                              : "text-gray-600"
                          }`}
                        >
                          {msg.timestamp.toLocaleString()}
                        </div>
                      </div>
                    ))
                  )}
                </div>
              )}
            </div>
          </aside>

          {/* Main Chat Area */}
          <main className="flex-1 flex flex-col overflow-hidden">
            <div className="flex-1 flex flex-col max-w-5xl mx-auto w-full px-6 overflow-hidden">
              {/* Dashboard Header */}
              <div className="py-6 space-y-2 flex-shrink-0">
                <div
                  className={`inline-flex border px-4 py-2 ${
                    theme === "light"
                      ? "border-black/20 bg-white"
                      : "border-white/10 bg-black/40"
                  }`}
                >
                  <span
                    className={`text-[10px] uppercase tracking-wider font-mono ${
                      theme === "light" ? "text-gray-600" : "text-gray-400"
                    }`}
                  >
                    Interactive Reputation Assistant
                  </span>
                </div>

                <h1 className="text-3xl md:text-4xl font-bold tracking-tight">
                  Dashboard
                </h1>
              </div>

              {/* Chat Messages Container */}
              <div
                className={`flex-1 border overflow-y-auto mb-6 min-h-0 ${
                  theme === "light"
                    ? "border-black/10 bg-gray-50/50"
                    : "border-white/5 bg-white/[0.02]"
                }`}
              >
                <div className="p-6 space-y-4">
                  {messages.map((message) => (
                    <MessageBubble
                      key={message.id}
                      message={message}
                      theme={theme}
                      onToggleBookmark={() => toggleBookmark(message.id)}
                    />
                  ))}

                  {/* Typing Indicator */}
                  {isTyping && (
                    <div className="flex items-start gap-3">
                      <div
                        className={`border p-2 ${
                          theme === "light"
                            ? "border-black/20 bg-white"
                            : "border-white/10 bg-black/40"
                        }`}
                      >
                        <Bot className="w-5 h-5" />
                      </div>
                      <div
                        className={`border px-4 py-3 ${
                          theme === "light"
                            ? "border-black/10 bg-white"
                            : "border-white/5 bg-black/20"
                        }`}
                      >
                        <div className="flex gap-1">
                          <div
                            className={`w-2 h-2 rounded-full animate-bounce ${
                              theme === "light" ? "bg-gray-400" : "bg-gray-600"
                            }`}
                            style={{ animationDelay: "0ms" }}
                          />
                          <div
                            className={`w-2 h-2 rounded-full animate-bounce ${
                              theme === "light" ? "bg-gray-400" : "bg-gray-600"
                            }`}
                            style={{ animationDelay: "150ms" }}
                          />
                          <div
                            className={`w-2 h-2 rounded-full animate-bounce ${
                              theme === "light" ? "bg-gray-400" : "bg-gray-600"
                            }`}
                            style={{ animationDelay: "300ms" }}
                          />
                        </div>
                      </div>
                    </div>
                  )}

                  <div ref={messagesEndRef} />
                </div>
              </div>

              {/* Input Area - Venture Style */}
              <div
                className={`border p-4 flex-shrink-0 ${
                  theme === "light"
                    ? "border-black/10 bg-white"
                    : "border-white/5 bg-black/20"
                }`}
              >
                <div className="flex gap-3">
                  <input
                    type="text"
                    value={inputValue}
                    onChange={(e) => setInputValue(e.target.value)}
                    onKeyPress={handleKeyPress}
                    placeholder="Ask about your reputation, governance, staking..."
                    className={`flex-1 border px-4 py-3 text-sm bg-transparent focus:outline-none focus:ring-2 focus:ring-offset-2 transition-all ${
                      theme === "light"
                        ? "border-black/20 placeholder-gray-500 focus:ring-black/20 focus:border-black/40"
                        : "border-white/10 placeholder-gray-600 focus:ring-white/20 focus:border-white/30"
                    }`}
                  />
                  <button
                    onClick={handleSendMessage}
                    disabled={!inputValue.trim()}
                    className={`border px-6 py-3 transition-all disabled:opacity-30 disabled:cursor-not-allowed ${
                      theme === "light"
                        ? "border-black/20 hover:bg-black/5 hover:border-black/40"
                        : "border-white/10 hover:bg-white/5 hover:border-white/30"
                    }`}
                  >
                    <Send className="w-5 h-5" />
                  </button>
                </div>

                {/* Quick Actions */}
                <div className="flex flex-wrap gap-2 mt-4">
                  <QuickActionButton
                    theme={theme}
                    label="My Reputation"
                    onClick={() => setInputValue("Show my reputation score")}
                  />
                  <QuickActionButton
                    theme={theme}
                    label="Governance"
                    onClick={() =>
                      setInputValue(
                        "Show my governance activity of Your_Address"
                      )
                    }
                  />
                  <QuickActionButton
                    theme={theme}
                    label="Staking"
                    onClick={() =>
                      setInputValue("Show staking points of Your_Address")
                    }
                  />
                  <QuickActionButton
                    theme={theme}
                    label="Identity Verification"
                    onClick={() =>
                      setInputValue("Analyze my identity verification")
                    }
                  />
                  <QuickActionButton
                    theme={theme}
                    label="Leaderboard"
                    onClick={() => setInputValue("Compare with leaderboard")}
                  />
                  <QuickActionButton
                    theme={theme}
                    label="Historical Trends"
                    onClick={() => setInputValue("View historical trends")}
                  />
                  <QuickActionButton
                    theme={theme}
                    label="Recommendations"
                    onClick={() =>
                      setInputValue("Get personalized recommendations")
                    }
                  />
                  <QuickActionButton
                    theme={theme}
                    label="Help"
                    onClick={() => setInputValue("What can you help me with?")}
                  />
                </div>
              </div>
            </div>
          </main>
        </div>
      </div>
    </div>
  );
}

// Message Bubble Component
function MessageBubble({
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

// Reputation Data Display Component
function ReputationDataDisplay({
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

// Score Item Component
function ScoreItem({
  icon,
  label,
  score,
  max,
  theme,
}: {
  icon: React.ReactNode;
  label: string;
  score: number;
  max: number;
  theme: "light" | "dark";
}) {
  return (
    <div
      className={`border p-3 ${
        theme === "light"
          ? "border-black/10 bg-gray-50"
          : "border-white/5 bg-black/20"
      }`}
    >
      <div className="flex items-center gap-2 mb-2">
        <div className={theme === "light" ? "text-gray-600" : "text-gray-500"}>
          {icon}
        </div>
        <span
          className={`text-xs uppercase tracking-wider font-mono ${
            theme === "light" ? "text-gray-600" : "text-gray-500"
          }`}
        >
          {label}
        </span>
      </div>
      <div className="flex items-baseline gap-1">
        <span className="text-xl font-bold">{score}</span>
        <span
          className={`text-sm ${theme === "light" ? "text-gray-600" : "text-gray-500"}`}
        >
          /{max}
        </span>
      </div>
    </div>
  );
}

// Quick Action Button Component
function QuickActionButton({
  theme,
  label,
  onClick,
}: {
  theme: "light" | "dark";
  label: string;
  onClick: () => void;
}) {
  return (
    <button
      onClick={onClick}
      className={`border px-3 py-1.5 text-xs uppercase tracking-wider font-medium transition-colors ${
        theme === "light"
          ? "border-black/20 hover:bg-black/5 text-gray-700 hover:text-black"
          : "border-white/10 hover:bg-white/5 text-gray-400 hover:text-white"
      }`}
    >
      {label}
    </button>
  );
}
