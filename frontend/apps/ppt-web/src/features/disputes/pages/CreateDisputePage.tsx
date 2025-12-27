/**
 * CreateDisputePage (Epic 77: Dispute Resolution).
 */
import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import type { CreateDisputeRequest, DisputeCategory } from '../types';
import { CATEGORY_LABELS } from '../types';

export function CreateDisputePage() {
  const navigate = useNavigate();
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [formData, setFormData] = useState<CreateDisputeRequest>({
    respondentId: '',
    title: '',
    description: '',
    category: 'other',
    desiredResolution: '',
  });

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>
  ) => {
    const { name, value } = e.target;
    setFormData((prev) => ({ ...prev, [name]: value }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);
    setError(null);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1000));
      navigate('/disputes');
    } catch {
      setError('Failed to create dispute. Please try again.');
    } finally {
      setIsSubmitting(false);
    }
  };

  const categories = Object.entries(CATEGORY_LABELS) as [DisputeCategory, string][];

  return (
    <div className="max-w-2xl mx-auto px-4 py-8">
      <div className="mb-8">
        <Link to="/disputes" className="text-blue-600 hover:text-blue-800">
          Back to Disputes
        </Link>
        <h1 className="text-2xl font-bold mt-4">File New Dispute</h1>
        <p className="text-gray-600 mt-2">
          Submit a dispute for resolution. Provide as much detail as possible.
        </p>
      </div>

      {error && (
        <div className="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded mb-6">
          {error}
        </div>
      )}

      <form onSubmit={handleSubmit} className="bg-white rounded-lg shadow p-6 space-y-6">
        <div>
          <label htmlFor="respondentId" className="block text-sm font-medium text-gray-700 mb-1">
            Respondent ID
          </label>
          <input
            type="text"
            id="respondentId"
            name="respondentId"
            value={formData.respondentId}
            onChange={handleChange}
            required
            className="w-full px-3 py-2 border border-gray-300 rounded-lg"
            placeholder="Enter the ID of the other party"
          />
        </div>

        <div>
          <label htmlFor="title" className="block text-sm font-medium text-gray-700 mb-1">
            Dispute Title
          </label>
          <input
            type="text"
            id="title"
            name="title"
            value={formData.title}
            onChange={handleChange}
            required
            className="w-full px-3 py-2 border border-gray-300 rounded-lg"
            placeholder="Brief title for your dispute"
          />
        </div>

        <div>
          <label htmlFor="category" className="block text-sm font-medium text-gray-700 mb-1">
            Category
          </label>
          <select
            id="category"
            name="category"
            value={formData.category}
            onChange={handleChange}
            className="w-full px-3 py-2 border border-gray-300 rounded-lg"
          >
            {categories.map(([value, label]) => (
              <option key={value} value={value}>
                {label}
              </option>
            ))}
          </select>
        </div>

        <div>
          <label htmlFor="description" className="block text-sm font-medium text-gray-700 mb-1">
            Description
          </label>
          <textarea
            id="description"
            name="description"
            value={formData.description}
            onChange={handleChange}
            required
            rows={5}
            className="w-full px-3 py-2 border border-gray-300 rounded-lg"
            placeholder="Describe the dispute in detail..."
          />
        </div>

        <div>
          <label
            htmlFor="desiredResolution"
            className="block text-sm font-medium text-gray-700 mb-1"
          >
            Desired Resolution
          </label>
          <textarea
            id="desiredResolution"
            name="desiredResolution"
            value={formData.desiredResolution}
            onChange={handleChange}
            required
            rows={3}
            className="w-full px-3 py-2 border border-gray-300 rounded-lg"
            placeholder="What outcome are you seeking?"
          />
        </div>

        <div className="flex justify-end gap-4 pt-4">
          <Link
            to="/disputes"
            className="px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50"
          >
            Cancel
          </Link>
          <button
            type="submit"
            disabled={isSubmitting}
            className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50"
          >
            {isSubmitting ? 'Submitting...' : 'Submit Dispute'}
          </button>
        </div>
      </form>
    </div>
  );
}
