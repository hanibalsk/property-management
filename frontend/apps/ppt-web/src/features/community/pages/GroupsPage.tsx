/**
 * GroupsPage
 *
 * Main page for browsing and managing community groups.
 * Part of Story 42.1: Community Groups.
 */

import type { CommunityGroupSummary, ListGroupsParams } from '@ppt/api-client';
import { useState } from 'react';
import { GroupList } from '../components/GroupList';

interface GroupsPageProps {
  groups: CommunityGroupSummary[];
  total: number;
  isLoading?: boolean;
  joiningGroupId?: string;
  leavingGroupId?: string;
  onNavigateToCreate: () => void;
  onNavigateToGroup: (id: string) => void;
  onJoinGroup: (id: string) => void;
  onLeaveGroup: (id: string) => void;
  onFilterChange: (params: ListGroupsParams) => void;
}

export function GroupsPage({
  groups,
  total,
  isLoading,
  joiningGroupId,
  leavingGroupId,
  onNavigateToCreate,
  onNavigateToGroup,
  onJoinGroup,
  onLeaveGroup,
  onFilterChange,
}: GroupsPageProps) {
  const [page, setPage] = useState(1);
  const [pageSize] = useState(12);

  const handlePageChange = (newPage: number) => {
    setPage(newPage);
    onFilterChange({ page: newPage, pageSize });
  };

  const handleFilterChange = (params: ListGroupsParams) => {
    setPage(params.page || 1);
    onFilterChange({ ...params, pageSize });
  };

  return (
    <div className="max-w-7xl mx-auto px-4 py-8">
      <GroupList
        groups={groups}
        total={total}
        page={page}
        pageSize={pageSize}
        isLoading={isLoading}
        joiningGroupId={joiningGroupId}
        leavingGroupId={leavingGroupId}
        onPageChange={handlePageChange}
        onFilterChange={handleFilterChange}
        onView={onNavigateToGroup}
        onJoin={onJoinGroup}
        onLeave={onLeaveGroup}
        onCreate={onNavigateToCreate}
      />
    </div>
  );
}
