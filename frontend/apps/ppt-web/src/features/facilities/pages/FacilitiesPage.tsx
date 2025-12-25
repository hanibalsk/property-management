/**
 * Facilities Page (Epic 56: Facility Booking).
 *
 * Lists all facilities in a building with filtering options.
 */

import type { FacilitySummary, FacilityType, ListFacilitiesQuery } from '@ppt/api-client';
import { listFacilities } from '@ppt/api-client';
import { useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { FacilityList } from '../components';

const PAGE_SIZE = 10;

// TODO: Replace with useAuth() hook when auth context is implemented
// This controls visibility of manager-only features (create, edit facilities)
// Backend still enforces authorization - this is for UI only
function useIsManager(): boolean {
  // TODO: Implement proper role check - get user role from auth context
  // Example: const { user } = useAuth(); return user?.role === 'manager' || user?.role === 'admin';
  // For now, return true to enable manager features during development
  // In production, this should check actual user role from JWT claims
  return true;
}

export function FacilitiesPage() {
  const { buildingId } = useParams<{ buildingId: string }>();
  const navigate = useNavigate();
  const isManager = useIsManager();

  const [facilities, setFacilities] = useState<FacilitySummary[]>([]);
  const [total, setTotal] = useState(0);
  const [isLoading, setIsLoading] = useState(true);
  const [filters, setFilters] = useState<ListFacilitiesQuery>({});

  useEffect(() => {
    if (!buildingId) return;

    const fetchFacilities = async () => {
      setIsLoading(true);
      try {
        const response = await listFacilities(buildingId, filters);
        setFacilities(response.items);
        setTotal(response.total);
      } catch (error) {
        console.error('Failed to fetch facilities:', error);
      } finally {
        setIsLoading(false);
      }
    };

    fetchFacilities();
  }, [buildingId, filters]);

  const handleTypeFilter = (type?: FacilityType) => {
    setFilters((prev) => ({ ...prev, facility_type: type }));
  };

  const handleBookableFilter = (bookable?: boolean) => {
    setFilters((prev) => ({ ...prev, is_bookable: bookable }));
  };

  const handleView = (id: string) => {
    navigate(`/buildings/${buildingId}/facilities/${id}`);
  };

  const handleBook = (id: string) => {
    navigate(`/buildings/${buildingId}/facilities/${id}/book`);
  };

  const handleEdit = (id: string) => {
    navigate(`/buildings/${buildingId}/facilities/${id}/edit`);
  };

  const handleCreate = () => {
    navigate(`/buildings/${buildingId}/facilities/new`);
  };

  return (
    <div className="max-w-6xl mx-auto px-4 py-8">
      <FacilityList
        facilities={facilities}
        total={total}
        page={1}
        pageSize={PAGE_SIZE}
        isLoading={isLoading}
        isManager={isManager}
        onPageChange={() => {}}
        onTypeFilter={handleTypeFilter}
        onBookableFilter={handleBookableFilter}
        onView={handleView}
        onBook={handleBook}
        onEdit={handleEdit}
        onCreate={handleCreate}
      />
    </div>
  );
}
