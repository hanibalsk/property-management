/**
 * Content Moderation Dashboard Page (Epic 67, Story 67.4).
 *
 * Dashboard for reviewing and moderating user-generated content.
 */

import type React from 'react';
import { useCallback, useEffect, useState } from 'react';
import { ModerationCaseCard } from '../components/ModerationCaseCard';
import type { ModerationCase, ViolationType } from '../components/ModerationCaseCard';
import { ModerationQueueStats } from '../components/ModerationQueueStats';
import type { ModerationQueueStatsData } from '../components/ModerationQueueStats';

interface ActionTemplate {
  id: string;
  name: string;
  violation_type: ViolationType;
  action_type: string;
  rationale_template: string;
  notify_owner: boolean;
}

export const ContentModerationPage: React.FC = () => {
  const [cases, setCases] = useState<ModerationCase[]>([]);
  const [stats, setStats] = useState<ModerationQueueStatsData | null>(null);
  const [templates, setTemplates] = useState<ActionTemplate[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Filters
  const [statusFilter, setStatusFilter] = useState<string>('pending');
  const [contentTypeFilter, setContentTypeFilter] = useState<string>('');
  const [violationTypeFilter, setViolationTypeFilter] = useState<string>('');
  const [priorityFilter, setPriorityFilter] = useState<string>('');
  const [unassignedOnly, setUnassignedOnly] = useState(false);

  // Load data
  useEffect(() => {
    const loadData = async () => {
      try {
        setIsLoading(true);
        setError(null);

        // In production, these would be API calls
        await new Promise((resolve) => setTimeout(resolve, 500));

        setStats({
          pending_count: 0,
          under_review_count: 0,
          by_priority: [
            { priority: 1, count: 0 },
            { priority: 2, count: 0 },
            { priority: 3, count: 0 },
          ],
          by_violation_type: [],
          avg_resolution_time_hours: 0,
          overdue_count: 0,
        });

        setTemplates([
          {
            id: '1',
            name: 'Remove Spam',
            violation_type: 'spam',
            action_type: 'remove',
            rationale_template: 'Content removed as it violates our spam policy.',
            notify_owner: true,
          },
          {
            id: '2',
            name: 'Warn for Inappropriate Language',
            violation_type: 'inappropriate_content',
            action_type: 'warn',
            rationale_template: 'Warning issued for use of inappropriate language.',
            notify_owner: true,
          },
        ]);

        setCases([]);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load moderation data');
        console.error('Failed to load moderation data:', err);
      } finally {
        setIsLoading(false);
      }
    };

    loadData();
  }, [statusFilter, contentTypeFilter, violationTypeFilter, priorityFilter, unassignedOnly]);

  const handleAssign = useCallback((caseId: string) => {
    console.log('Assign case:', caseId);
    // API call to assign case
  }, []);

  const handleTakeAction = useCallback((caseId: string) => {
    console.log('Take action on case:', caseId);
    // Open action modal
  }, []);

  const handleViewContent = useCallback((caseId: string) => {
    console.log('View content for case:', caseId);
    // Navigate to content view
  }, []);

  const handleDecideAppeal = useCallback((caseId: string) => {
    console.log('Decide appeal for case:', caseId);
    // Open appeal decision modal
  }, []);

  const handleFilterByPriority = useCallback((priority: number) => {
    setPriorityFilter(priority.toString());
  }, []);

  const handleFilterByViolationType = useCallback((type: string) => {
    setViolationTypeFilter(type);
  }, []);

  const handleShowOverdue = useCallback(() => {
    // Filter to show overdue cases
    setStatusFilter('pending');
    console.log('Show overdue cases');
  }, []);

  const filteredCases = cases.filter((c) => {
    if (statusFilter && c.status !== statusFilter) return false;
    if (contentTypeFilter && c.content_type !== contentTypeFilter) return false;
    if (violationTypeFilter && c.violation_type !== violationTypeFilter) return false;
    if (priorityFilter && c.priority.toString() !== priorityFilter) return false;
    if (unassignedOnly && c.assigned_to_name) return false;
    return true;
  });

  return (
    <div className="content-moderation-page">
      <div className="moderation-page-header">
        <h1>Content Moderation Dashboard</h1>
        <p>Review and moderate user-generated content for DSA compliance.</p>
      </div>

      {error && (
        <div className="moderation-page-error" role="alert">
          {error}
        </div>
      )}

      {/* Queue Statistics */}
      {stats && (
        <ModerationQueueStats
          stats={stats}
          onFilterByPriority={handleFilterByPriority}
          onFilterByViolationType={handleFilterByViolationType}
          onShowOverdue={handleShowOverdue}
        />
      )}

      {/* Filters */}
      <div className="moderation-filters-section">
        <h2>Moderation Queue</h2>
        <div className="moderation-filters">
          <div className="moderation-filter">
            <label htmlFor="statusFilter">Status</label>
            <select
              id="statusFilter"
              value={statusFilter}
              onChange={(e) => setStatusFilter(e.target.value)}
            >
              <option value="">All Statuses</option>
              <option value="pending">Pending</option>
              <option value="under_review">Under Review</option>
              <option value="appealed">Appealed</option>
              <option value="removed">Removed</option>
              <option value="restricted">Restricted</option>
              <option value="warned">Warned</option>
              <option value="approved">Approved</option>
            </select>
          </div>
          <div className="moderation-filter">
            <label htmlFor="contentTypeFilter">Content Type</label>
            <select
              id="contentTypeFilter"
              value={contentTypeFilter}
              onChange={(e) => setContentTypeFilter(e.target.value)}
            >
              <option value="">All Types</option>
              <option value="listing">Listing</option>
              <option value="listing_photo">Listing Photo</option>
              <option value="review">Review</option>
              <option value="comment">Comment</option>
              <option value="community_post">Community Post</option>
              <option value="message">Message</option>
            </select>
          </div>
          <div className="moderation-filter">
            <label htmlFor="violationTypeFilter">Violation Type</label>
            <select
              id="violationTypeFilter"
              value={violationTypeFilter}
              onChange={(e) => setViolationTypeFilter(e.target.value)}
            >
              <option value="">All Violations</option>
              <option value="spam">Spam</option>
              <option value="harassment">Harassment</option>
              <option value="hate_speech">Hate Speech</option>
              <option value="violence">Violence</option>
              <option value="illegal_content">Illegal Content</option>
              <option value="misinformation">Misinformation</option>
              <option value="fraud">Fraud</option>
              <option value="privacy">Privacy</option>
              <option value="inappropriate_content">Inappropriate Content</option>
            </select>
          </div>
          <div className="moderation-filter">
            <label htmlFor="priorityFilter">Priority</label>
            <select
              id="priorityFilter"
              value={priorityFilter}
              onChange={(e) => setPriorityFilter(e.target.value)}
            >
              <option value="">All Priorities</option>
              <option value="1">Critical (P1)</option>
              <option value="2">High (P2)</option>
              <option value="3">Medium (P3)</option>
              <option value="4">Low (P4)</option>
              <option value="5">Lowest (P5)</option>
            </select>
          </div>
          <div className="moderation-filter checkbox">
            <label htmlFor="unassignedOnly">
              <input
                type="checkbox"
                id="unassignedOnly"
                checked={unassignedOnly}
                onChange={(e) => setUnassignedOnly(e.target.checked)}
              />
              Unassigned Only
            </label>
          </div>
        </div>
      </div>

      {/* Cases List */}
      {isLoading ? (
        <div className="moderation-loading">Loading moderation queue...</div>
      ) : filteredCases.length > 0 ? (
        <div className="moderation-cases-list">
          {filteredCases.map((case_) => (
            <ModerationCaseCard
              key={case_.id}
              case_={case_}
              onAssign={handleAssign}
              onTakeAction={handleTakeAction}
              onViewContent={handleViewContent}
              onDecideAppeal={handleDecideAppeal}
              showActions={true}
              isModerator={true}
            />
          ))}
        </div>
      ) : (
        <div className="moderation-empty-state">
          <p>No moderation cases found matching the criteria.</p>
          <p>The moderation queue is currently empty.</p>
        </div>
      )}

      {/* Action Templates */}
      <div className="moderation-templates-section">
        <h2>Action Templates</h2>
        <div className="moderation-templates-list">
          {templates.map((template) => (
            <div key={template.id} className="moderation-template-card">
              <h4>{template.name}</h4>
              <p className="template-violation">{template.violation_type}</p>
              <p className="template-action">{template.action_type}</p>
              <p className="template-rationale">{template.rationale_template}</p>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

ContentModerationPage.displayName = 'ContentModerationPage';
