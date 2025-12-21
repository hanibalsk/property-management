import type { AnnouncementTargetType } from '@ppt/api-client';
import { useEffect, useState } from 'react';

interface TargetSelectorProps {
  targetType: AnnouncementTargetType;
  targetIds: string[];
  onTargetTypeChange: (type: AnnouncementTargetType) => void;
  onTargetIdsChange: (ids: string[]) => void;
  buildings?: { id: string; name: string }[];
  units?: { id: string; name: string; buildingName: string }[];
  roles?: { id: string; name: string }[];
}

export function TargetSelector({
  targetType,
  targetIds,
  onTargetTypeChange,
  onTargetIdsChange,
  buildings = [],
  units = [],
  roles = [],
}: TargetSelectorProps) {
  const [searchTerm, setSearchTerm] = useState('');

  useEffect(() => {
    // Clear target IDs when type changes
    if (targetType === 'all') {
      onTargetIdsChange([]);
    }
  }, [targetType, onTargetIdsChange]);

  const handleToggleItem = (id: string) => {
    if (targetIds.includes(id)) {
      onTargetIdsChange(targetIds.filter((i) => i !== id));
    } else {
      onTargetIdsChange([...targetIds, id]);
    }
  };

  const renderTargetList = () => {
    switch (targetType) {
      case 'building': {
        const filtered = buildings.filter((b) =>
          b.name.toLowerCase().includes(searchTerm.toLowerCase())
        );
        return (
          <div className="mt-2 max-h-48 overflow-y-auto border rounded-md">
            {filtered.length === 0 ? (
              <p className="p-3 text-gray-500 text-sm">No buildings found</p>
            ) : (
              filtered.map((building) => (
                <label
                  key={building.id}
                  className="flex items-center p-2 hover:bg-gray-50 cursor-pointer"
                >
                  <input
                    type="checkbox"
                    checked={targetIds.includes(building.id)}
                    onChange={() => handleToggleItem(building.id)}
                    className="mr-2"
                  />
                  <span className="text-sm">{building.name}</span>
                </label>
              ))
            )}
          </div>
        );
      }

      case 'units': {
        const filtered = units.filter(
          (u) =>
            u.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
            u.buildingName.toLowerCase().includes(searchTerm.toLowerCase())
        );
        return (
          <div className="mt-2 max-h-48 overflow-y-auto border rounded-md">
            {filtered.length === 0 ? (
              <p className="p-3 text-gray-500 text-sm">No units found</p>
            ) : (
              filtered.map((unit) => (
                <label
                  key={unit.id}
                  className="flex items-center p-2 hover:bg-gray-50 cursor-pointer"
                >
                  <input
                    type="checkbox"
                    checked={targetIds.includes(unit.id)}
                    onChange={() => handleToggleItem(unit.id)}
                    className="mr-2"
                  />
                  <span className="text-sm">
                    {unit.name} <span className="text-gray-400">({unit.buildingName})</span>
                  </span>
                </label>
              ))
            )}
          </div>
        );
      }

      case 'roles': {
        const filtered = roles.filter((r) =>
          r.name.toLowerCase().includes(searchTerm.toLowerCase())
        );
        return (
          <div className="mt-2 max-h-48 overflow-y-auto border rounded-md">
            {filtered.length === 0 ? (
              <p className="p-3 text-gray-500 text-sm">No roles found</p>
            ) : (
              filtered.map((role) => (
                <label
                  key={role.id}
                  className="flex items-center p-2 hover:bg-gray-50 cursor-pointer"
                >
                  <input
                    type="checkbox"
                    checked={targetIds.includes(role.id)}
                    onChange={() => handleToggleItem(role.id)}
                    className="mr-2"
                  />
                  <span className="text-sm">{role.name}</span>
                </label>
              ))
            )}
          </div>
        );
      }

      default:
        return null;
    }
  };

  return (
    <div>
      <span className="block text-sm font-medium text-gray-700 mb-2">Target Audience</span>

      <div className="flex flex-wrap gap-2 mb-3">
        {(['all', 'building', 'units', 'roles'] as const).map((type) => (
          <button
            key={type}
            type="button"
            onClick={() => onTargetTypeChange(type)}
            className={`px-3 py-1 rounded-full text-sm transition-colors ${
              targetType === type
                ? 'bg-blue-600 text-white'
                : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
            }`}
          >
            {type === 'all' && 'All Users'}
            {type === 'building' && 'Building'}
            {type === 'units' && 'Specific Units'}
            {type === 'roles' && 'By Role'}
          </button>
        ))}
      </div>

      {targetType !== 'all' && (
        <>
          <input
            type="text"
            placeholder={`Search ${targetType}...`}
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          {renderTargetList()}
          {targetIds.length > 0 && (
            <p className="mt-2 text-sm text-gray-500">
              {targetIds.length} {targetType === 'building' ? 'building(s)' : targetType} selected
            </p>
          )}
        </>
      )}
    </div>
  );
}
