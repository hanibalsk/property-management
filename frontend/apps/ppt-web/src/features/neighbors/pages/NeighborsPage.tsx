/**
 * NeighborsPage Component
 *
 * Main page for viewing neighbors in a building.
 * Presentational component - receives data as props.
 */

import type React from 'react';
import { useCallback, useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { NeighborCard } from '../components';
import type { NeighborView } from '../types';

export interface NeighborsPageProps {
  neighbors: NeighborView[];
  buildingName?: string;
  isLoading?: boolean;
  error?: string | null;
  onContact?: (neighbor: NeighborView) => void;
  onViewProfile?: (neighbor: NeighborView) => void;
  onManagePrivacy?: () => void;
}

type SortOption = 'name' | 'unit' | 'floor';

export const NeighborsPage: React.FC<NeighborsPageProps> = ({
  neighbors,
  buildingName,
  isLoading = false,
  error = null,
  onContact,
  onViewProfile,
  onManagePrivacy,
}) => {
  const { t } = useTranslation();
  const [searchQuery, setSearchQuery] = useState('');
  const [sortBy, setSortBy] = useState<SortOption>('name');
  const [filterFloor, setFilterFloor] = useState<string>('');

  // Get unique floors for filter dropdown
  const floors = useMemo(() => {
    const floorSet = new Set<number>();
    for (const neighbor of neighbors) {
      if (neighbor.floor !== undefined) {
        floorSet.add(neighbor.floor);
      }
    }
    return Array.from(floorSet).sort((a, b) => a - b);
  }, [neighbors]);

  // Filter and sort neighbors
  const filteredNeighbors = useMemo(() => {
    let result = [...neighbors];

    // Apply search filter
    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      result = result.filter((neighbor) => {
        const displayName =
          neighbor.displayName || [neighbor.firstName, neighbor.lastName].filter(Boolean).join(' ');
        return (
          displayName.toLowerCase().includes(query) ||
          neighbor.unitNumber?.toLowerCase().includes(query) ||
          neighbor.bio?.toLowerCase().includes(query)
        );
      });
    }

    // Apply floor filter
    if (filterFloor) {
      const floor = Number.parseInt(filterFloor, 10);
      result = result.filter((neighbor) => neighbor.floor === floor);
    }

    // Apply sorting
    result.sort((a, b) => {
      switch (sortBy) {
        case 'name': {
          const nameA = a.displayName || [a.firstName, a.lastName].filter(Boolean).join(' ') || '';
          const nameB = b.displayName || [b.firstName, b.lastName].filter(Boolean).join(' ') || '';
          return nameA.localeCompare(nameB);
        }
        case 'unit': {
          const unitA = a.unitNumber || '';
          const unitB = b.unitNumber || '';
          return unitA.localeCompare(unitB, undefined, { numeric: true });
        }
        case 'floor': {
          const floorA = a.floor ?? Number.MAX_VALUE;
          const floorB = b.floor ?? Number.MAX_VALUE;
          return floorA - floorB;
        }
        default:
          return 0;
      }
    });

    return result;
  }, [neighbors, searchQuery, filterFloor, sortBy]);

  const handleSearchChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchQuery(e.target.value);
  }, []);

  const handleSortChange = useCallback((e: React.ChangeEvent<HTMLSelectElement>) => {
    setSortBy(e.target.value as SortOption);
  }, []);

  const handleFloorChange = useCallback((e: React.ChangeEvent<HTMLSelectElement>) => {
    setFilterFloor(e.target.value);
  }, []);

  if (isLoading) {
    return (
      <div className="neighbors-page">
        <div className="neighbors-page-loading">
          <div className="neighbors-page-spinner" />
          <p>{t('neighbors.loading')}</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="neighbors-page">
        <div className="neighbors-page-error" role="alert">
          <h2>{t('common.error')}</h2>
          <p>{error}</p>
        </div>
      </div>
    );
  }

  return (
    <div className="neighbors-page">
      <div className="neighbors-page-header">
        <div className="neighbors-page-title-section">
          <h1>{t('neighbors.title')}</h1>
          {buildingName && <p className="neighbors-page-subtitle">{buildingName}</p>}
          <p className="neighbors-page-description">{t('neighbors.description')}</p>
        </div>
        {onManagePrivacy && (
          <button type="button" onClick={onManagePrivacy} className="neighbors-page-privacy-button">
            {t('neighbors.managePrivacy')}
          </button>
        )}
      </div>

      <div className="neighbors-page-controls">
        <div className="neighbors-page-filters">
          <div className="neighbors-page-filter">
            <label htmlFor="neighbors-search">{t('common.search')}</label>
            <input
              type="search"
              id="neighbors-search"
              value={searchQuery}
              onChange={handleSearchChange}
              placeholder={t('neighbors.searchPlaceholder')}
              className="neighbors-page-search"
            />
          </div>

          {floors.length > 0 && (
            <div className="neighbors-page-filter">
              <label htmlFor="neighbors-floor">{t('neighbors.floor')}</label>
              <select
                id="neighbors-floor"
                value={filterFloor}
                onChange={handleFloorChange}
                className="neighbors-page-select"
              >
                <option value="">{t('neighbors.allFloors')}</option>
                {floors.map((floor) => (
                  <option key={floor} value={floor}>
                    {t('neighbors.floorNumber', { floor })}
                  </option>
                ))}
              </select>
            </div>
          )}

          <div className="neighbors-page-filter">
            <label htmlFor="neighbors-sort">{t('neighbors.sortBy')}</label>
            <select
              id="neighbors-sort"
              value={sortBy}
              onChange={handleSortChange}
              className="neighbors-page-select"
            >
              <option value="name">{t('neighbors.sortByName')}</option>
              <option value="unit">{t('neighbors.sortByUnit')}</option>
              <option value="floor">{t('neighbors.sortByFloor')}</option>
            </select>
          </div>
        </div>

        <div className="neighbors-page-count">
          {t('neighbors.showing', { count: filteredNeighbors.length, total: neighbors.length })}
        </div>
      </div>

      {filteredNeighbors.length === 0 ? (
        <div className="neighbors-page-empty">
          {searchQuery || filterFloor ? (
            <>
              <h2>{t('neighbors.noResults')}</h2>
              <p>{t('neighbors.noResultsDescription')}</p>
            </>
          ) : (
            <>
              <h2>{t('neighbors.noNeighbors')}</h2>
              <p>{t('neighbors.noNeighborsDescription')}</p>
            </>
          )}
        </div>
      ) : (
        <div className="neighbors-page-grid">
          {filteredNeighbors.map((neighbor) => (
            <NeighborCard
              key={neighbor.id}
              neighbor={neighbor}
              onContact={onContact}
              onViewProfile={onViewProfile}
              showActions={true}
            />
          ))}
        </div>
      )}
    </div>
  );
};

NeighborsPage.displayName = 'NeighborsPage';
