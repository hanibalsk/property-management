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

/**
 * Hook to determine if the current user has manager privileges.
 *
 * This controls visibility of manager-only features (create, edit facilities).
 * Note: Backend still enforces authorization - this is for UI display only.
 *
 * Integration steps when AuthContext is implemented:
 * 1. Import useAuth from the auth context
 * 2. Extract user role from the authenticated user
 * 3. Return true if role is 'manager' or 'admin'
 *
 * Example implementation:
 * ```typescript
 * function useIsManager(): boolean {
 *   const { user } = useAuth();
 *   return user?.role === 'manager' || user?.role === 'admin';
 * }
 * ```
 *
 * @returns true if user has manager privileges, false otherwise
 */
function useIsManager(): boolean {
  // Placeholder: Returns true during development to enable all manager features.
  // Replace with actual role check when AuthContext is implemented for ppt-web.
  // See frontend/apps/mobile/src/contexts/AuthContext.tsx for reference implementation.
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
        onPageChange={(_page) => {
          // TODO: Implement pagination for facilities
        }}
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
