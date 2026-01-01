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
  const [page, setPage] = useState(1);
  const [isLoading, setIsLoading] = useState(true);
  const [filters, setFilters] = useState<Omit<ListFacilitiesQuery, 'limit' | 'offset'>>({});

  useEffect(() => {
    if (!buildingId) return;

    const fetchFacilities = async () => {
      setIsLoading(true);
      try {
        const offset = (page - 1) * PAGE_SIZE;
        const response = await listFacilities(buildingId, {
          ...filters,
          limit: PAGE_SIZE,
          offset,
        });
        setFacilities(response.items);
        setTotal(response.total);
      } catch (error) {
        console.error('Failed to fetch facilities:', error);
      } finally {
        setIsLoading(false);
      }
    };

    fetchFacilities();
  }, [buildingId, filters, page]);

  const handleTypeFilter = (type?: FacilityType) => {
    setFilters((prev) => ({ ...prev, facility_type: type }));
    setPage(1); // Reset to first page when filter changes
  };

  const handleBookableFilter = (bookable?: boolean) => {
    setFilters((prev) => ({ ...prev, is_bookable: bookable }));
    setPage(1); // Reset to first page when filter changes
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

  const handlePageChange = (newPage: number) => {
    setPage(newPage);
  };

  return (
    <div className="max-w-6xl mx-auto px-4 py-8">
      <FacilityList
        facilities={facilities}
        total={total}
        page={page}
        pageSize={PAGE_SIZE}
        isLoading={isLoading}
        isManager={isManager}
        onPageChange={handlePageChange}
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
