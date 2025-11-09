/**
 * DotRepute Dashboard Page - Interactive Chat Interface with Sidebar
 * Venture-style dashboard with bot conversation flow
 * Features: Chat history, Bookmarks, Expandable messages
 */

'use client';

import { useEffect, useState, useRef } from 'react';
import { VentureNavbar } from '@/components/venture-navbar';
import Link from 'next/link';
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
  MoreVertical
} from 'lucide-react';

// Message type definition
interface Message {
  id: string;
  role: 'user' | 'bot';
  content: string;
  timestamp: Date;
  type?: 'text' | 'data' | 'question';
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
  const [theme, setTheme] = useState<'light' | 'dark'>('dark');
  const [mounted, setMounted] = useState(false);
  const [messages, setMessages] = useState<Message[]>([]);
  const [inputValue, setInputValue] = useState('');
  const [isTyping, setIsTyping] = useState(false);
  const [isLoggedIn, setIsLoggedIn] = useState(true); // Default to logged in on dashboard
  const [sidebarTab, setSidebarTab] = useState<'chats' | 'bookmarks'>('chats');
  const [chatSessions, setChatSessions] = useState<ChatSession[]>([]);
  const [currentSessionId, setCurrentSessionId] = useState<string>('1');
  const [sessionMessages, setSessionMessages] = useState<Record<string, Message[]>>({});
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Initialize theme from localStorage on mount
  useEffect(() => {
    setMounted(true);
    const savedTheme = localStorage.getItem('theme') as 'light' | 'dark' | null;
    const initialTheme = savedTheme || 'dark';
    setTheme(initialTheme);
    document.documentElement.classList.toggle('dark', initialTheme === 'dark');

    // Load saved chat sessions from localStorage
    const savedSessions = localStorage.getItem('chatSessions');
    const savedSessionMessages = localStorage.getItem('sessionMessages');
    const savedCurrentSession = localStorage.getItem('currentSessionId');

    if (savedSessions && savedSessionMessages) {
      const sessions = JSON.parse(savedSessions);
      const messagesMap = JSON.parse(savedSessionMessages);

      // Parse dates back from strings
      sessions.forEach((session: ChatSession) => {
        session.timestamp = new Date(session.timestamp);
      });

      Object.keys(messagesMap).forEach(sessionId => {
        messagesMap[sessionId].forEach((msg: Message) => {
          msg.timestamp = new Date(msg.timestamp);
        });
      });

      setChatSessions(sessions);
      setSessionMessages(messagesMap);

      const currentId = savedCurrentSession || sessions[0]?.id || '1';
      setCurrentSessionId(currentId);
      setMessages(messagesMap[currentId] || []);
    } else {
      // Initial bot greeting for new users
      const initialMessage: Message = {
        id: '1',
        role: 'bot',
        content: 'Welcome to DotRepute! I\'m your reputation assistant. I can help you check your reputation score, analyze contributions, view governance participation, and more. What would you like to know?',
        timestamp: new Date(),
        type: 'text'
      };

      const initialSession: ChatSession = {
        id: '1',
        title: 'New Chat',
        lastMessage: 'Welcome to DotRepute!',
        timestamp: new Date(),
        messageCount: 1
      };

      setMessages([initialMessage]);
      setChatSessions([initialSession]);
      setSessionMessages({ '1': [initialMessage] });
    }
  }, []);

  // Auto-scroll to bottom when new messages arrive
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  // Save chat sessions to localStorage whenever they change
  useEffect(() => {
    if (mounted && chatSessions.length > 0) {
      localStorage.setItem('chatSessions', JSON.stringify(chatSessions));
      localStorage.setItem('sessionMessages', JSON.stringify(sessionMessages));
      localStorage.setItem('currentSessionId', currentSessionId);
    }
  }, [chatSessions, sessionMessages, currentSessionId, mounted]);

  // Toggle theme and persist to localStorage
  const toggleTheme = () => {
    const newTheme = theme === 'dark' ? 'light' : 'dark';
    setTheme(newTheme);
    localStorage.setItem('theme', newTheme);
    document.documentElement.classList.toggle('dark', newTheme === 'dark');
  };

  // Handle logout
  const handleLogout = () => {
    // Optionally clear chat history on logout
    // localStorage.removeItem('chatSessions');
    // localStorage.removeItem('sessionMessages');
    // localStorage.removeItem('currentSessionId');

    setIsLoggedIn(false);
    // Redirect to auth page
    window.location.href = '/auth';
  };

  // Create new chat session
  const createNewChat = () => {
    const newSessionId = Date.now().toString();
    const initialMessage: Message = {
      id: Date.now().toString(),
      role: 'bot',
      content: 'Hello! How can I assist you with your reputation today?',
      timestamp: new Date(),
      type: 'text'
    };

    const newSession: ChatSession = {
      id: newSessionId,
      title: 'New Chat',
      lastMessage: '',
      timestamp: new Date(),
      messageCount: 1
    };

    setChatSessions(prev => [newSession, ...prev]);
    setCurrentSessionId(newSessionId);
    setMessages([initialMessage]);
    setSessionMessages(prev => ({
      ...prev,
      [newSessionId]: [initialMessage]
    }));
  };

  // Switch to a different chat session
  const switchSession = (sessionId: string) => {
    setCurrentSessionId(sessionId);
    setMessages(sessionMessages[sessionId] || []);
  };

  // Toggle bookmark on message
  const toggleBookmark = (messageId: string) => {
    setMessages(prev =>
      prev.map(msg =>
        msg.id === messageId
          ? { ...msg, isBookmarked: !msg.isBookmarked }
          : msg
      )
    );
  };

  // Get bookmarked messages
  const bookmarkedMessages = messages.filter(msg => msg.isBookmarked);

  // Bot response logic - Understanding and responding
  const generateBotResponse = async (userMessage: string): Promise<Message> => {
    const lowerMessage = userMessage.toLowerCase();

    // Check for reputation score query
    if (lowerMessage.includes('reputation') || lowerMessage.includes('score')) {
      return {
        id: Date.now().toString(),
        role: 'bot',
        content: 'Here\'s your current reputation score breakdown:',
        timestamp: new Date(),
        type: 'data',
        data: {
          totalScore: 847,
          maxScore: 1000,
          rank: 234,
          percentile: 'Top 5%',
          breakdown: {
            identity: { score: 92, max: 100 },
            governance: { score: 78, max: 100 },
            staking: { score: 85, max: 100 },
            activity: { score: 71, max: 100 }
          }
        }
      };
    }

    // Check for identity query
    if (lowerMessage.includes('identity')) {
      return {
        id: Date.now().toString(),
        role: 'bot',
        content: 'Your identity score is 92/100. This is based on your verified on-chain identity. You\'ve completed all verification steps including display name, legal name, and email verification.\n\nHere are the details:\n• Display Name: Verified ✓\n• Legal Name: Verified ✓\n• Email: Verified ✓\n• Twitter: Verified ✓\n• Web: Verified ✓\n\nYour identity is fully verified on-chain, which significantly boosts your reputation score.',
        timestamp: new Date(),
        type: 'text'
      };
    }

    // Check for governance query
    if (lowerMessage.includes('governance') || lowerMessage.includes('vote')) {
      return {
        id: Date.now().toString(),
        role: 'bot',
        content: 'Your governance participation score is 78/100. You\'ve voted on 15 referenda in the past 30 days.\n\nRecent Voting Activity:\n• Referendum #245 - Treasury Proposal (Aye)\n• Referendum #243 - System Upgrade (Aye)\n• Referendum #240 - Community Grant (Nay)\n• Referendum #238 - Network Parameter (Aye)\n• Referendum #235 - Technical Update (Aye)\n\nYour participation rate is excellent! Keep engaging with governance to maintain your high score.',
        timestamp: new Date(),
        type: 'text'
      };
    }

    // Check for staking query
    if (lowerMessage.includes('staking') || lowerMessage.includes('stake')) {
      return {
        id: Date.now().toString(),
        role: 'bot',
        content: 'Your staking score is 85/100. You currently have 500 DOT staked in nomination pools.\n\nStaking Details:\n• Total Staked: 500 DOT\n• Pool: Nomination Pool #42\n• APY: ~14.5%\n• Duration: 6 months\n• Rewards Earned: 43.5 DOT\n\nThis represents strong network commitment and contributes significantly to your reputation score.',
        timestamp: new Date(),
        type: 'text'
      };
    }

    // Check for activity query
    if (lowerMessage.includes('activity') || lowerMessage.includes('contribution')) {
      return {
        id: Date.now().toString(),
        role: 'bot',
        content: 'Your activity score is 71/100. You\'ve made 24 on-chain transactions this month.\n\nRecent Activities:\n• Voted on Referendum #245 (2 hours ago)\n• Increased Stake by 100 DOT (1 day ago)\n• GitHub Contribution - Merged PR to polkadot-sdk (3 days ago)\n• Identity Update - Added Twitter handle (1 week ago)\n• Treasury Proposal Discussion (2 weeks ago)\n\nYour consistent engagement across governance, staking, and development shows strong ecosystem participation.',
        timestamp: new Date(),
        type: 'text'
      };
    }

    // Check for help or what can you do
    if (lowerMessage.includes('help') || lowerMessage.includes('what can')) {
      return {
        id: Date.now().toString(),
        role: 'bot',
        content: 'I can help you with:\n\n• Check your reputation score\n• Analyze identity verification\n• View governance participation\n• Track staking activity\n• Monitor recent contributions\n• Compare with leaderboard\n• View historical trends\n• Get personalized recommendations\n\nJust ask me anything about your reputation! You can also bookmark important messages by clicking the bookmark icon.',
        timestamp: new Date(),
        type: 'text'
      };
    }

    // Default response - ask for clarification
    return {
      id: Date.now().toString(),
      role: 'bot',
      content: 'I\'m not sure I understood that. Could you please clarify? You can ask me about your reputation score, identity, governance, staking, or activity.',
      timestamp: new Date(),
      type: 'question'
    };
  };

  // Handle sending message
  const handleSendMessage = async () => {
    if (!inputValue.trim()) return;

    // Add user message
    const userMessage: Message = {
      id: Date.now().toString(),
      role: 'user',
      content: inputValue,
      timestamp: new Date(),
      type: 'text'
    };

    const updatedMessages = [...messages, userMessage];
    setMessages(updatedMessages);

    // Update session messages
    setSessionMessages(prev => ({
      ...prev,
      [currentSessionId]: updatedMessages
    }));

    // Update chat session
    setChatSessions(prev =>
      prev.map(session =>
        session.id === currentSessionId
          ? {
              ...session,
              lastMessage: inputValue.substring(0, 50) + (inputValue.length > 50 ? '...' : ''),
              timestamp: new Date(),
              messageCount: session.messageCount + 1,
              title: session.messageCount === 0 ? inputValue.substring(0, 30) + (inputValue.length > 30 ? '...' : '') : session.title
            }
          : session
      )
    );

    setInputValue('');
    setIsTyping(true);

    // Simulate bot thinking time
    await new Promise(resolve => setTimeout(resolve, 1000));

    // Generate bot response
    const botResponse = await generateBotResponse(userMessage.content);
    const finalMessages = [...updatedMessages, botResponse];
    setMessages(finalMessages);

    // Save bot response to session messages
    setSessionMessages(prev => ({
      ...prev,
      [currentSessionId]: finalMessages
    }));

    setIsTyping(false);
  };

  // Handle Enter key press
  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

  // Prevent flash of unstyled content
  if (!mounted) return null;

  return (
    <div className={`relative min-h-screen transition-colors duration-300 ${
      theme === 'light' ? 'bg-white text-black' : 'bg-black text-white'
    }`}>
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
          <aside className={`w-80 border-r flex flex-col transition-colors ${
            theme === 'light'
              ? 'border-black/10 bg-gray-50'
              : 'border-white/5 bg-black'
          }`}>
          {/* Sidebar Header - Login/Signup or User Info */}
          <div className={`p-4 border-b ${
            theme === 'light' ? 'border-black/10' : 'border-white/5'
          }`}>
            {!isLoggedIn ? (
              <div className="space-y-3">
                <p className={`text-sm ${
                  theme === 'light' ? 'text-gray-600' : 'text-gray-400'
                }`}>
                  Sign in to save your chat history
                </p>
                <div className="flex gap-2">
                  <Link
                    href="/auth"
                    className={`flex-1 border px-3 py-2 text-xs uppercase tracking-wider font-medium transition-colors text-center ${
                      theme === 'light'
                        ? 'border-black/20 hover:bg-black/5 text-gray-700'
                        : 'border-white/10 hover:bg-white/5 text-gray-400'
                    }`}
                  >
                    <LogIn className="w-4 h-4 inline mr-2" />
                    Login
                  </Link>
                  <Link
                    href="/auth"
                    className={`flex-1 border px-3 py-2 text-xs uppercase tracking-wider font-medium transition-colors text-center ${
                      theme === 'light'
                        ? 'border-black/20 bg-black text-white hover:bg-black/90'
                        : 'border-white/10 bg-white text-black hover:bg-white/90'
                    }`}
                  >
                    <UserPlus className="w-4 h-4 inline mr-2" />
                    Sign Up
                  </Link>
                </div>
              </div>
            ) : (
              <div className="flex items-center gap-3">
                <div className={`border p-2 ${
                  theme === 'light'
                    ? 'border-black/20 bg-white'
                    : 'border-white/10 bg-black/40'
                }`}>
                  <User className="w-5 h-5" />
                </div>
                <div className="flex-1">
                  <div className="font-medium">User Account</div>
                  <div className={`text-xs ${
                    theme === 'light' ? 'text-gray-600' : 'text-gray-500'
                  }`}>
                    user@polkadot.network
                  </div>
                </div>
              </div>
            )}
          </div>

          {/* Tab Switcher */}
          <div className={`flex border-b ${
            theme === 'light' ? 'border-black/10' : 'border-white/5'
          }`}>
            <button
              onClick={() => setSidebarTab('chats')}
              className={`flex-1 px-4 py-3 text-xs uppercase tracking-wider font-medium transition-colors ${
                sidebarTab === 'chats'
                  ? theme === 'light'
                    ? 'border-b-2 border-black bg-white text-black'
                    : 'border-b-2 border-white bg-black text-white'
                  : theme === 'light'
                    ? 'text-gray-600 hover:bg-gray-100'
                    : 'text-gray-500 hover:bg-white/5'
              }`}
            >
              <MessageSquare className="w-4 h-4 inline mr-2" />
              Chats
            </button>
            <button
              onClick={() => setSidebarTab('bookmarks')}
              className={`flex-1 px-4 py-3 text-xs uppercase tracking-wider font-medium transition-colors ${
                sidebarTab === 'bookmarks'
                  ? theme === 'light'
                    ? 'border-b-2 border-black bg-white text-black'
                    : 'border-b-2 border-white bg-black text-white'
                  : theme === 'light'
                    ? 'text-gray-600 hover:bg-gray-100'
                    : 'text-gray-500 hover:bg-white/5'
              }`}
            >
              <Bookmark className="w-4 h-4 inline mr-2" />
              Saved ({bookmarkedMessages.length})
            </button>
          </div>

          {/* Sidebar Content */}
          <div className="flex-1 overflow-y-auto">
            {sidebarTab === 'chats' ? (
              <div className="p-3 space-y-2">
                {/* New Chat Button */}
                <button
                  onClick={createNewChat}
                  className={`w-full border p-3 text-sm transition-colors flex items-center gap-3 ${
                    theme === 'light'
                      ? 'border-black/20 hover:bg-white hover:border-black/30'
                      : 'border-white/10 hover:bg-white/5 hover:border-white/20'
                  }`}
                >
                  <Plus className="w-4 h-4" />
                  <span className="font-medium">New Chat</span>
                </button>

                {/* Chat Sessions List */}
                {chatSessions.map(session => (
                  <button
                    key={session.id}
                    onClick={() => switchSession(session.id)}
                    className={`w-full border p-3 text-left transition-colors ${
                      session.id === currentSessionId
                        ? theme === 'light'
                          ? 'border-black/30 bg-orange-50'
                          : 'border-orange-900/30 bg-orange-950/20'
                        : theme === 'light'
                          ? 'border-black/10 hover:bg-white hover:border-black/20'
                          : 'border-white/5 hover:bg-white/5 hover:border-white/10'
                    }`}
                  >
                    <div className="flex items-start justify-between gap-2">
                      <div className="flex-1 min-w-0">
                        <div className="font-medium text-sm truncate mb-1">
                          {session.title}
                        </div>
                        <div className={`text-xs truncate ${
                          theme === 'light' ? 'text-gray-600' : 'text-gray-500'
                        }`}>
                          {session.lastMessage || 'No messages yet'}
                        </div>
                        <div className={`text-xs font-mono mt-1 ${
                          theme === 'light' ? 'text-gray-500' : 'text-gray-600'
                        }`}>
                          {session.timestamp.toLocaleDateString()}
                        </div>
                      </div>
                      <div className={`text-xs font-mono ${
                        theme === 'light' ? 'text-gray-500' : 'text-gray-600'
                      }`}>
                        {session.messageCount}
                      </div>
                    </div>
                  </button>
                ))}
              </div>
            ) : (
              <div className="p-3 space-y-2">
                {bookmarkedMessages.length === 0 ? (
                  <div className={`text-center py-8 text-sm ${
                    theme === 'light' ? 'text-gray-600' : 'text-gray-500'
                  }`}>
                    <Star className="w-8 h-8 mx-auto mb-2 opacity-50" />
                    No bookmarked messages yet
                  </div>
                ) : (
                  bookmarkedMessages.map(msg => (
                    <div
                      key={msg.id}
                      className={`border p-3 text-sm ${
                        theme === 'light'
                          ? 'border-black/10 bg-white'
                          : 'border-white/5 bg-black/20'
                      }`}
                    >
                      <div className="flex items-start gap-2 mb-2">
                        <div className={theme === 'light' ? 'text-gray-600' : 'text-gray-500'}>
                          {msg.role === 'bot' ? <Bot className="w-4 h-4" /> : <User className="w-4 h-4" />}
                        </div>
                        <div className="flex-1 min-w-0">
                          <p className="line-clamp-3">{msg.content}</p>
                        </div>
                      </div>
                      <div className={`text-xs font-mono ${
                        theme === 'light' ? 'text-gray-500' : 'text-gray-600'
                      }`}>
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
              <div className={`inline-flex border px-4 py-2 ${
                theme === 'light'
                  ? 'border-black/20 bg-white'
                  : 'border-white/10 bg-black/40'
              }`}>
                <span className={`text-[10px] uppercase tracking-wider font-mono ${
                  theme === 'light' ? 'text-gray-600' : 'text-gray-400'
                }`}>
                  Interactive Reputation Assistant
                </span>
              </div>

              <h1 className="text-3xl md:text-4xl font-bold tracking-tight">
                Dashboard
              </h1>
            </div>

            {/* Chat Messages Container */}
            <div className={`flex-1 border overflow-y-auto mb-6 min-h-0 ${
              theme === 'light'
                ? 'border-black/10 bg-gray-50/50'
                : 'border-white/5 bg-white/[0.02]'
            }`}>
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
                    <div className={`border p-2 ${
                      theme === 'light'
                        ? 'border-black/20 bg-white'
                        : 'border-white/10 bg-black/40'
                    }`}>
                      <Bot className="w-5 h-5" />
                    </div>
                    <div className={`border px-4 py-3 ${
                      theme === 'light'
                        ? 'border-black/10 bg-white'
                        : 'border-white/5 bg-black/20'
                    }`}>
                      <div className="flex gap-1">
                        <div className={`w-2 h-2 rounded-full animate-bounce ${
                          theme === 'light' ? 'bg-gray-400' : 'bg-gray-600'
                        }`} style={{ animationDelay: '0ms' }} />
                        <div className={`w-2 h-2 rounded-full animate-bounce ${
                          theme === 'light' ? 'bg-gray-400' : 'bg-gray-600'
                        }`} style={{ animationDelay: '150ms' }} />
                        <div className={`w-2 h-2 rounded-full animate-bounce ${
                          theme === 'light' ? 'bg-gray-400' : 'bg-gray-600'
                        }`} style={{ animationDelay: '300ms' }} />
                      </div>
                    </div>
                  </div>
                )}

                <div ref={messagesEndRef} />
              </div>
            </div>

            {/* Input Area - Venture Style */}
            <div className={`border p-4 flex-shrink-0 ${
              theme === 'light'
                ? 'border-black/10 bg-white'
                : 'border-white/5 bg-black/20'
            }`}>
              <div className="flex gap-3">
                <input
                  type="text"
                  value={inputValue}
                  onChange={(e) => setInputValue(e.target.value)}
                  onKeyPress={handleKeyPress}
                  placeholder="Ask about your reputation, governance, staking..."
                  className={`flex-1 border px-4 py-3 text-sm bg-transparent focus:outline-none focus:ring-2 focus:ring-offset-2 transition-all ${
                    theme === 'light'
                      ? 'border-black/20 placeholder-gray-500 focus:ring-black/20 focus:border-black/40'
                      : 'border-white/10 placeholder-gray-600 focus:ring-white/20 focus:border-white/30'
                  }`}
                />
                <button
                  onClick={handleSendMessage}
                  disabled={!inputValue.trim()}
                  className={`border px-6 py-3 transition-all disabled:opacity-30 disabled:cursor-not-allowed ${
                    theme === 'light'
                      ? 'border-black/20 hover:bg-black/5 hover:border-black/40'
                      : 'border-white/10 hover:bg-white/5 hover:border-white/30'
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
                  onClick={() => setInputValue('Show my reputation score')}
                />
                <QuickActionButton
                  theme={theme}
                  label="Governance"
                  onClick={() => setInputValue('Show my governance activity')}
                />
                <QuickActionButton
                  theme={theme}
                  label="Staking"
                  onClick={() => setInputValue('What is my staking score?')}
                />
                <QuickActionButton
                  theme={theme}
                  label="Help"
                  onClick={() => setInputValue('What can you help me with?')}
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
  onToggleBookmark
}: {
  message: Message;
  theme: 'light' | 'dark';
  onToggleBookmark: () => void;
}) {
  const isUser = message.role === 'user';

  return (
    <div className={`flex items-start gap-3 ${isUser ? 'flex-row-reverse' : ''}`}>
      {/* Avatar */}
      <div className={`border p-2 flex-shrink-0 ${
        theme === 'light'
          ? 'border-black/20 bg-white'
          : 'border-white/10 bg-black/40'
      }`}>
        {isUser ? (
          <User className="w-5 h-5" />
        ) : (
          <Bot className="w-5 h-5" />
        )}
      </div>

      {/* Message Content */}
      <div className={`flex-1 min-w-0 ${isUser ? 'flex flex-col items-end' : ''}`}>
        <div className={`border px-4 py-3 w-full ${
          isUser
            ? theme === 'light'
              ? 'border-black/20 bg-orange-50'
              : 'border-orange-900/30 bg-orange-950/20'
            : theme === 'light'
              ? 'border-black/10 bg-white'
              : 'border-white/5 bg-black/20'
        }`}>
          {message.type === 'data' && message.data ? (
            <ReputationDataDisplay data={message.data} theme={theme} content={message.content} />
          ) : (
            <p className="text-sm leading-relaxed whitespace-pre-line break-words">{message.content}</p>
          )}

          {/* Message Actions */}
          {!isUser && (
            <div className="flex items-center gap-2 mt-3 pt-3 border-t border-current border-opacity-10">
              <button
                onClick={onToggleBookmark}
                className={`transition-colors ${
                  message.isBookmarked
                    ? 'text-yellow-500'
                    : theme === 'light'
                      ? 'text-gray-400 hover:text-gray-700'
                      : 'text-gray-600 hover:text-gray-400'
                }`}
              >
                <Bookmark className={`w-4 h-4 ${message.isBookmarked ? 'fill-current' : ''}`} />
              </button>
            </div>
          )}
        </div>

        {/* Timestamp */}
        <span className={`text-xs font-mono mt-1 ${
          theme === 'light' ? 'text-gray-500' : 'text-gray-600'
        }`}>
          {message.timestamp.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
        </span>
      </div>
    </div>
  );
}

// Reputation Data Display Component
function ReputationDataDisplay({ data, theme, content }: { data: any; theme: 'light' | 'dark'; content: string }) {
  return (
    <div className="space-y-4">
      <p className="text-sm">{content}</p>

      {/* Score Overview */}
      <div className="space-y-2">
        <div className="flex items-baseline gap-3">
          <span className="text-4xl font-bold">{data.totalScore}</span>
          <span className={theme === 'light' ? 'text-gray-600' : 'text-gray-500'}>/ {data.maxScore}</span>
        </div>

        <div className={`border h-2 overflow-hidden ${
          theme === 'light'
            ? 'border-black/20 bg-gray-100'
            : 'border-white/10 bg-white/5'
        }`}>
          <div
            className={`h-full ${
              theme === 'light'
                ? 'bg-gradient-to-r from-orange-500 to-yellow-500'
                : 'bg-gradient-to-r from-orange-400 to-yellow-400'
            }`}
            style={{ width: `${(data.totalScore / data.maxScore) * 100}%` }}
          />
        </div>

        <div className="flex gap-4 text-sm">
          <span className={theme === 'light' ? 'text-gray-600' : 'text-gray-500'}>
            Rank: <span className="font-bold">#{data.rank}</span>
          </span>
          <span className={theme === 'light' ? 'text-gray-600' : 'text-gray-500'}>
            {data.percentile}
          </span>
        </div>
      </div>

      {/* Breakdown */}
      <div className="grid grid-cols-2 gap-3 pt-2">
        <ScoreItem icon={<Shield className="w-4 h-4" />} label="Identity" score={data.breakdown.identity.score} max={data.breakdown.identity.max} theme={theme} />
        <ScoreItem icon={<Vote className="w-4 h-4" />} label="Governance" score={data.breakdown.governance.score} max={data.breakdown.governance.max} theme={theme} />
        <ScoreItem icon={<Coins className="w-4 h-4" />} label="Staking" score={data.breakdown.staking.score} max={data.breakdown.staking.max} theme={theme} />
        <ScoreItem icon={<Activity className="w-4 h-4" />} label="Activity" score={data.breakdown.activity.score} max={data.breakdown.activity.max} theme={theme} />
      </div>
    </div>
  );
}

// Score Item Component
function ScoreItem({ icon, label, score, max, theme }: { icon: React.ReactNode; label: string; score: number; max: number; theme: 'light' | 'dark' }) {
  return (
    <div className={`border p-3 ${
      theme === 'light'
        ? 'border-black/10 bg-gray-50'
        : 'border-white/5 bg-black/20'
    }`}>
      <div className="flex items-center gap-2 mb-2">
        <div className={theme === 'light' ? 'text-gray-600' : 'text-gray-500'}>
          {icon}
        </div>
        <span className={`text-xs uppercase tracking-wider font-mono ${
          theme === 'light' ? 'text-gray-600' : 'text-gray-500'
        }`}>
          {label}
        </span>
      </div>
      <div className="flex items-baseline gap-1">
        <span className="text-xl font-bold">{score}</span>
        <span className={`text-sm ${theme === 'light' ? 'text-gray-600' : 'text-gray-500'}`}>/{max}</span>
      </div>
    </div>
  );
}

// Quick Action Button Component
function QuickActionButton({ theme, label, onClick }: { theme: 'light' | 'dark'; label: string; onClick: () => void }) {
  return (
    <button
      onClick={onClick}
      className={`border px-3 py-1.5 text-xs uppercase tracking-wider font-medium transition-colors ${
        theme === 'light'
          ? 'border-black/20 hover:bg-black/5 text-gray-700 hover:text-black'
          : 'border-white/10 hover:bg-white/5 text-gray-400 hover:text-white'
      }`}
    >
      {label}
    </button>
  );
}
