/**
 * FaultCard component for mobile - displays a fault summary card.
 * Epic 4: Fault Reporting & Resolution (UC-03)
 */

import { StyleSheet, Text, TouchableOpacity, View } from 'react-native';

export type FaultStatus =
  | 'new'
  | 'triaged'
  | 'in_progress'
  | 'waiting_parts'
  | 'scheduled'
  | 'resolved'
  | 'closed'
  | 'reopened';

export type FaultPriority = 'low' | 'medium' | 'high' | 'urgent';

export type FaultCategory =
  | 'plumbing'
  | 'electrical'
  | 'heating'
  | 'structural'
  | 'exterior'
  | 'elevator'
  | 'common_area'
  | 'security'
  | 'cleaning'
  | 'other';

export interface FaultSummary {
  id: string;
  buildingId: string;
  unitId?: string;
  title: string;
  category: FaultCategory;
  priority: FaultPriority;
  status: FaultStatus;
  createdAt: string;
}

interface FaultCardProps {
  fault: FaultSummary;
  onPress?: () => void;
}

const statusColors: Record<FaultStatus, { bg: string; text: string }> = {
  new: { bg: '#FEE2E2', text: '#991B1B' },
  triaged: { bg: '#DBEAFE', text: '#1E40AF' },
  in_progress: { bg: '#FEF3C7', text: '#92400E' },
  waiting_parts: { bg: '#FFEDD5', text: '#9A3412' },
  scheduled: { bg: '#E9D5FF', text: '#7C3AED' },
  resolved: { bg: '#D1FAE5', text: '#065F46' },
  closed: { bg: '#F3F4F6', text: '#374151' },
  reopened: { bg: '#FEE2E2', text: '#991B1B' },
};

const priorityColors: Record<FaultPriority, string> = {
  low: '#6B7280',
  medium: '#3B82F6',
  high: '#F97316',
  urgent: '#DC2626',
};

const categoryLabels: Record<FaultCategory, string> = {
  plumbing: 'Plumbing',
  electrical: 'Electrical',
  heating: 'Heating',
  structural: 'Structural',
  exterior: 'Exterior',
  elevator: 'Elevator',
  common_area: 'Common Area',
  security: 'Security',
  cleaning: 'Cleaning',
  other: 'Other',
};

const statusLabels: Record<FaultStatus, string> = {
  new: 'New',
  triaged: 'Triaged',
  in_progress: 'In Progress',
  waiting_parts: 'Waiting',
  scheduled: 'Scheduled',
  resolved: 'Resolved',
  closed: 'Closed',
  reopened: 'Reopened',
};

const priorityLabels: Record<FaultPriority, string> = {
  low: 'Low',
  medium: 'Medium',
  high: 'High',
  urgent: 'Urgent',
};

export function FaultCard({ fault, onPress }: FaultCardProps) {
  const statusColor = statusColors[fault.status];

  return (
    <TouchableOpacity style={styles.card} onPress={onPress} activeOpacity={0.7}>
      <View style={styles.header}>
        <Text style={styles.title} numberOfLines={2}>
          {fault.title}
        </Text>
        {fault.priority === 'urgent' && <Text style={styles.urgentIcon}>⚠️</Text>}
      </View>

      <View style={styles.badges}>
        <View style={[styles.badge, { backgroundColor: statusColor.bg }]}>
          <Text style={[styles.badgeText, { color: statusColor.text }]}>
            {statusLabels[fault.status]}
          </Text>
        </View>
        <Text style={[styles.priority, { color: priorityColors[fault.priority] }]}>
          {priorityLabels[fault.priority]}
        </Text>
        <Text style={styles.category}>{categoryLabels[fault.category]}</Text>
      </View>

      <Text style={styles.date}>Reported: {new Date(fault.createdAt).toLocaleDateString()}</Text>
    </TouchableOpacity>
  );
}

const styles = StyleSheet.create({
  card: {
    backgroundColor: '#FFFFFF',
    borderRadius: 12,
    padding: 16,
    marginHorizontal: 16,
    marginVertical: 8,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
  header: {
    flexDirection: 'row',
    alignItems: 'flex-start',
    justifyContent: 'space-between',
  },
  title: {
    fontSize: 16,
    fontWeight: '600',
    color: '#111827',
    flex: 1,
    marginRight: 8,
  },
  urgentIcon: {
    fontSize: 16,
  },
  badges: {
    flexDirection: 'row',
    alignItems: 'center',
    flexWrap: 'wrap',
    marginTop: 8,
    gap: 8,
  },
  badge: {
    paddingHorizontal: 8,
    paddingVertical: 4,
    borderRadius: 4,
  },
  badgeText: {
    fontSize: 12,
    fontWeight: '500',
  },
  priority: {
    fontSize: 12,
    fontWeight: '500',
  },
  category: {
    fontSize: 12,
    color: '#6B7280',
  },
  date: {
    fontSize: 12,
    color: '#9CA3AF',
    marginTop: 8,
  },
});
