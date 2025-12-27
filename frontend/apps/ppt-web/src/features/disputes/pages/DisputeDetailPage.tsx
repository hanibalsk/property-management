/**
 * DisputeDetailPage (Epic 77: Dispute Resolution).
 */
import { useState } from 'react';
import { Link } from 'react-router-dom';
import type { DisputeStatus, MessageType } from '../types';
import { CATEGORY_LABELS, PRIORITY_LABELS, STATUS_LABELS } from '../types';

interface DisputeDetailPageProps {
  disputeId: string;
}

interface DisputeMessage {
  id: string;
  senderId: string;
  senderName: string;
  messageType: MessageType;
  content: string;
  createdAt: string;
}

interface DisputeDetail {
  id: string;
  title: string;
  description: string;
  category: 'deposit' | 'maintenance' | 'noise' | 'fees' | 'contract' | 'neighbor' | 'other';
  status: DisputeStatus;
  priority: 'low' | 'medium' | 'high' | 'urgent';
  filedByName: string;
  respondentName: string;
  mediatorName?: string;
  desiredResolution: string;
  responseDeadline: string;
  createdAt: string;
  messages: DisputeMessage[];
}

const mockDispute: DisputeDetail = {
  id: '1',
  title: 'Security Deposit Return',
  description: 'My security deposit of $2,000 has not been returned.',
  category: 'deposit',
  status: 'pending_response',
  priority: 'high',
  filedByName: 'John Tenant',
  respondentName: 'Property Manager Inc.',
  desiredResolution: 'Full refund of the $2,000 security deposit within 14 days.',
  responseDeadline: '2024-12-15',
  createdAt: '2024-12-01',
  messages: [
    {
      id: '1',
      senderId: 'user1',
      senderName: 'John Tenant',
      messageType: 'text',
      content: 'I have attached photos showing the condition when I moved out.',
      createdAt: '2024-12-01T10:00:00Z',
    },
    {
      id: '2',
      senderId: 'system',
      senderName: 'System',
      messageType: 'system',
      content: 'Dispute filed. Awaiting response from Property Manager Inc.',
      createdAt: '2024-12-01T10:01:00Z',
    },
  ],
};

export function DisputeDetailPage({ disputeId }: DisputeDetailPageProps) {
  const [newMessage, setNewMessage] = useState('');
  const [isSending, setIsSending] = useState(false);

  const dispute = mockDispute;

  const handleSendMessage = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newMessage.trim()) return;
    setIsSending(true);
    try {
      await new Promise((resolve) => setTimeout(resolve, 500));
      setNewMessage('');
    } finally {
      setIsSending(false);
    }
  };

  const getStatusColor = (status: DisputeStatus) => {
    const colors: Record<DisputeStatus, string> = {
      filed: 'bg-blue-100 text-blue-800',
      pending_response: 'bg-yellow-100 text-yellow-800',
      under_review: 'bg-purple-100 text-purple-800',
      mediation: 'bg-indigo-100 text-indigo-800',
      resolution_proposed: 'bg-cyan-100 text-cyan-800',
      agreement_pending: 'bg-orange-100 text-orange-800',
      resolved: 'bg-green-100 text-green-800',
      escalated: 'bg-red-100 text-red-800',
      closed: 'bg-gray-100 text-gray-800',
      cancelled: 'bg-gray-100 text-gray-500',
    };
    return colors[status];
  };

  const formatDate = (dateStr: string) => {
    return new Date(dateStr).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    });
  };

  return (
    <div className="max-w-4xl mx-auto px-4 py-8">
      <div className="mb-6">
        <Link to="/disputes" className="text-blue-600 hover:text-blue-800">
          Back to Disputes
        </Link>
      </div>

      <div className="bg-white rounded-lg shadow p-6 mb-6">
        <div className="flex items-start justify-between mb-4">
          <div>
            <h1 className="text-2xl font-bold">{dispute.title}</h1>
            <p className="text-gray-500 text-sm mt-1">Dispute #{disputeId}</p>
          </div>
          <span
            className={`px-3 py-1 rounded-full text-sm font-medium ${getStatusColor(dispute.status)}`}
          >
            {STATUS_LABELS[dispute.status]}
          </span>
        </div>

        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
          <div>
            <span className="text-gray-500">Category</span>
            <p className="font-medium">{CATEGORY_LABELS[dispute.category]}</p>
          </div>
          <div>
            <span className="text-gray-500">Priority</span>
            <p className="font-medium">{PRIORITY_LABELS[dispute.priority]}</p>
          </div>
          <div>
            <span className="text-gray-500">Filed By</span>
            <p className="font-medium">{dispute.filedByName}</p>
          </div>
          <div>
            <span className="text-gray-500">Respondent</span>
            <p className="font-medium">{dispute.respondentName}</p>
          </div>
        </div>
      </div>

      <div className="bg-white rounded-lg shadow p-6 mb-6">
        <h2 className="text-lg font-semibold mb-3">Description</h2>
        <p className="text-gray-700 whitespace-pre-wrap">{dispute.description}</p>

        <h3 className="text-md font-semibold mt-6 mb-2">Desired Resolution</h3>
        <p className="text-gray-700">{dispute.desiredResolution}</p>

        <div className="mt-6 pt-4 border-t flex gap-8 text-sm">
          <div>
            <span className="text-gray-500">Filed On</span>
            <p className="font-medium">{formatDate(dispute.createdAt)}</p>
          </div>
          <div>
            <span className="text-gray-500">Response Deadline</span>
            <p className="font-medium text-red-600">{formatDate(dispute.responseDeadline)}</p>
          </div>
        </div>
      </div>

      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-lg font-semibold mb-4">Communication Thread</h2>

        <div className="space-y-4 mb-6 max-h-96 overflow-y-auto">
          {dispute.messages.map((msg) => (
            <div
              key={msg.id}
              className={`p-4 rounded-lg ${
                msg.messageType === 'system' ? 'bg-gray-100 text-center' : 'bg-blue-50'
              }`}
            >
              {msg.messageType !== 'system' && (
                <div className="flex items-center justify-between mb-2">
                  <span className="font-medium">{msg.senderName}</span>
                  <span className="text-gray-500 text-sm">{formatDate(msg.createdAt)}</span>
                </div>
              )}
              <p className={msg.messageType === 'system' ? 'text-gray-600 text-sm' : ''}>
                {msg.content}
              </p>
            </div>
          ))}
        </div>

        <form onSubmit={handleSendMessage} className="border-t pt-4">
          <label htmlFor="newMessage" className="block text-sm font-medium text-gray-700 mb-2">
            Send Message
          </label>
          <textarea
            id="newMessage"
            value={newMessage}
            onChange={(e) => setNewMessage(e.target.value)}
            rows={3}
            className="w-full px-3 py-2 border border-gray-300 rounded-lg mb-3"
            placeholder="Type your message..."
          />
          <div className="flex justify-end">
            <button
              type="submit"
              disabled={isSending || !newMessage.trim()}
              className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50"
            >
              {isSending ? 'Sending...' : 'Send Message'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
