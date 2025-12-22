/**
 * CreateFaultPage - page for creating a new fault report.
 * Epic 4: Fault Reporting & Resolution (UC-03.1)
 */

import { FaultForm, type FaultFormData } from '../components/FaultForm';

interface CreateFaultPageProps {
  buildings: Array<{ id: string; name: string }>;
  units: Array<{ id: string; designation: string }>;
  isSubmitting?: boolean;
  onSubmit: (data: FaultFormData) => void;
  onCancel: () => void;
}

export function CreateFaultPage({
  buildings,
  units,
  isSubmitting,
  onSubmit,
  onCancel,
}: CreateFaultPageProps) {
  return (
    <div className="max-w-2xl mx-auto px-4 py-8">
      <div className="mb-6">
        <button
          type="button"
          onClick={onCancel}
          className="text-sm text-blue-600 hover:text-blue-800 flex items-center gap-1 mb-4"
        >
          ← Back to Faults
        </button>
        <h1 className="text-2xl font-bold text-gray-900">Report a Fault</h1>
        <p className="text-gray-600 mt-2">
          Describe the issue you're experiencing. Include as much detail as possible to help us
          resolve it quickly.
        </p>
      </div>

      <div className="bg-white rounded-lg shadow p-6">
        <FaultForm
          buildings={buildings}
          units={units}
          isSubmitting={isSubmitting}
          onSubmit={onSubmit}
          onCancel={onCancel}
        />
      </div>
    </div>
  );
}
