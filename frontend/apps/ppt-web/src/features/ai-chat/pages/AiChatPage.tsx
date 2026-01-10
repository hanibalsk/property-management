/**
 * AiChatPage - AI Assistant chat page
 * Epic 127: AI Chatbot Interface
 */

import { ChatInterface } from '../components';

export function AiChatPage() {
  return (
    <div className="container mx-auto px-4 py-6 h-[calc(100vh-120px)]">
      <div className="h-full">
        <ChatInterface showSidebar={true} />
      </div>
    </div>
  );
}
