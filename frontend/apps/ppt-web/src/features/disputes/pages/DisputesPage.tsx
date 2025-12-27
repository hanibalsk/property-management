/**
 * DisputesPage (Epic 77: Dispute Resolution).
 */
import { Link } from 'react-router-dom';

export function DisputesPage() {
  return (
    <div className="max-w-4xl mx-auto px-4 py-8">
      <div className="flex items-center justify-between mb-8">
        <h1 className="text-2xl font-bold">Dispute Resolution</h1>
        <Link
          to="/disputes/new"
          className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
        >
          File New Dispute
        </Link>
      </div>
      <div className="bg-white rounded-lg shadow p-6">
        <p className="text-gray-600">No disputes found. File a new dispute to get started.</p>
      </div>
    </div>
  );
}
