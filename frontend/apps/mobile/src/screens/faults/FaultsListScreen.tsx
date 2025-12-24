import { useCallback, useState } from 'react';
import { Pressable, RefreshControl, ScrollView, StyleSheet, Text, View } from 'react-native';

export type FaultStatus = 'open' | 'in_progress' | 'resolved' | 'closed';
export type FaultPriority = 'low' | 'medium' | 'high' | 'urgent';
export type FaultCategory =
  | 'plumbing'
  | 'electrical'
  | 'structural'
  | 'hvac'
  | 'elevator'
  | 'security'
  | 'other';

export interface Fault {
  id: string;
  title: string;
  description: string;
  status: FaultStatus;
  priority: FaultPriority;
  category: FaultCategory;
  location: string;
  createdAt: string;
  updatedAt: string;
  photos: string[];
  reportedBy: string;
  assignedTo?: string;
}

// Mock data
const mockFaults: Fault[] = [
  {
    id: '1',
    title: 'Leaking pipe in basement',
    description: 'Water is dripping from the ceiling pipe near the storage units',
    status: 'in_progress',
    priority: 'high',
    category: 'plumbing',
    location: 'Basement, Section B',
    createdAt: '2025-12-20T10:00:00Z',
    updatedAt: '2025-12-23T14:30:00Z',
    photos: [],
    reportedBy: 'John Doe',
    assignedTo: 'Maintenance Team',
  },
  {
    id: '2',
    title: 'Elevator stuck on 3rd floor',
    description: 'The main elevator is not responding and appears stuck',
    status: 'open',
    priority: 'urgent',
    category: 'elevator',
    location: 'Main lobby',
    createdAt: '2025-12-24T08:00:00Z',
    updatedAt: '2025-12-24T08:00:00Z',
    photos: [],
    reportedBy: 'Jane Smith',
  },
  {
    id: '3',
    title: 'Broken light in stairwell',
    description: 'Light bulb burnt out on 5th floor stairwell',
    status: 'resolved',
    priority: 'low',
    category: 'electrical',
    location: '5th floor stairwell',
    createdAt: '2025-12-18T16:00:00Z',
    updatedAt: '2025-12-22T11:00:00Z',
    photos: [],
    reportedBy: 'Mike Johnson',
    assignedTo: 'Electrician',
  },
];

interface FaultsListScreenProps {
  onNavigate?: (screen: string, params?: Record<string, unknown>) => void;
}

export function FaultsListScreen({ onNavigate }: FaultsListScreenProps) {
  const [refreshing, setRefreshing] = useState(false);
  const [faults] = useState<Fault[]>(mockFaults);
  const [filter, setFilter] = useState<'all' | 'open' | 'resolved'>('all');

  const onRefresh = useCallback(async () => {
    setRefreshing(true);
    await new Promise((resolve) => setTimeout(resolve, 1000));
    setRefreshing(false);
  }, []);

  const getStatusColor = (status: FaultStatus): string => {
    switch (status) {
      case 'open':
        return '#ef4444';
      case 'in_progress':
        return '#f59e0b';
      case 'resolved':
        return '#10b981';
      case 'closed':
        return '#6b7280';
      default:
        return '#6b7280';
    }
  };

  const getPriorityColor = (priority: FaultPriority): string => {
    switch (priority) {
      case 'urgent':
        return '#dc2626';
      case 'high':
        return '#ea580c';
      case 'medium':
        return '#ca8a04';
      case 'low':
        return '#65a30d';
      default:
        return '#6b7280';
    }
  };

  const getCategoryIcon = (category: FaultCategory): string => {
    switch (category) {
      case 'plumbing':
        return '🚿';
      case 'electrical':
        return '⚡';
      case 'structural':
        return '🏗️';
      case 'hvac':
        return '❄️';
      case 'elevator':
        return '🛗';
      case 'security':
        return '🔒';
      default:
        return '🔧';
    }
  };

  const formatDate = (dateString: string): string => {
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  const filteredFaults = faults.filter((fault) => {
    if (filter === 'all') return true;
    if (filter === 'open') return fault.status === 'open' || fault.status === 'in_progress';
    if (filter === 'resolved') return fault.status === 'resolved' || fault.status === 'closed';
    return true;
  });

  return (
    <View style={styles.container}>
      {/* Header */}
      <View style={styles.header}>
        <Text style={styles.headerTitle}>Faults</Text>
        <Pressable style={styles.addButton} onPress={() => onNavigate?.('ReportFault')}>
          <Text style={styles.addButtonText}>+ Report</Text>
        </Pressable>
      </View>

      {/* Filters */}
      <View style={styles.filters}>
        {(['all', 'open', 'resolved'] as const).map((option) => (
          <Pressable
            key={option}
            style={[styles.filterButton, filter === option && styles.filterButtonActive]}
            onPress={() => setFilter(option)}
          >
            <Text style={[styles.filterText, filter === option && styles.filterTextActive]}>
              {option.charAt(0).toUpperCase() + option.slice(1)}
            </Text>
          </Pressable>
        ))}
      </View>

      {/* Faults List */}
      <ScrollView
        style={styles.scrollView}
        refreshControl={
          <RefreshControl refreshing={refreshing} onRefresh={onRefresh} tintColor="#2563eb" />
        }
      >
        {filteredFaults.length === 0 ? (
          <View style={styles.emptyState}>
            <Text style={styles.emptyIcon}>🔧</Text>
            <Text style={styles.emptyTitle}>No faults found</Text>
            <Text style={styles.emptyText}>All issues have been addressed</Text>
          </View>
        ) : (
          filteredFaults.map((fault) => (
            <Pressable
              key={fault.id}
              style={styles.faultCard}
              onPress={() => onNavigate?.('FaultDetail', { faultId: fault.id })}
            >
              <View style={styles.faultHeader}>
                <Text style={styles.categoryIcon}>{getCategoryIcon(fault.category)}</Text>
                <View style={styles.faultTitleContainer}>
                  <Text style={styles.faultTitle} numberOfLines={1}>
                    {fault.title}
                  </Text>
                  <Text style={styles.faultLocation}>{fault.location}</Text>
                </View>
                <View
                  style={[styles.statusBadge, { backgroundColor: getStatusColor(fault.status) }]}
                >
                  <Text style={styles.statusText}>{fault.status.replace('_', ' ')}</Text>
                </View>
              </View>

              <Text style={styles.faultDescription} numberOfLines={2}>
                {fault.description}
              </Text>

              <View style={styles.faultFooter}>
                <View
                  style={[styles.priorityBadge, { borderColor: getPriorityColor(fault.priority) }]}
                >
                  <Text style={[styles.priorityText, { color: getPriorityColor(fault.priority) }]}>
                    {fault.priority}
                  </Text>
                </View>
                <Text style={styles.faultDate}>{formatDate(fault.updatedAt)}</Text>
              </View>

              {fault.assignedTo && (
                <View style={styles.assignedRow}>
                  <Text style={styles.assignedLabel}>Assigned to:</Text>
                  <Text style={styles.assignedValue}>{fault.assignedTo}</Text>
                </View>
              )}
            </Pressable>
          ))
        )}

        <View style={styles.bottomSpacer} />
      </ScrollView>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    padding: 20,
    paddingTop: 60,
    backgroundColor: '#fff',
    borderBottomWidth: 1,
    borderBottomColor: '#e5e7eb',
  },
  headerTitle: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#1f2937',
  },
  addButton: {
    backgroundColor: '#2563eb',
    paddingHorizontal: 16,
    paddingVertical: 8,
    borderRadius: 8,
  },
  addButtonText: {
    color: '#fff',
    fontWeight: '600',
    fontSize: 14,
  },
  filters: {
    flexDirection: 'row',
    padding: 16,
    gap: 8,
    backgroundColor: '#fff',
  },
  filterButton: {
    paddingHorizontal: 16,
    paddingVertical: 8,
    borderRadius: 20,
    backgroundColor: '#f3f4f6',
  },
  filterButtonActive: {
    backgroundColor: '#2563eb',
  },
  filterText: {
    fontSize: 14,
    color: '#6b7280',
    fontWeight: '500',
  },
  filterTextActive: {
    color: '#fff',
  },
  scrollView: {
    flex: 1,
    padding: 16,
  },
  emptyState: {
    alignItems: 'center',
    justifyContent: 'center',
    paddingVertical: 60,
  },
  emptyIcon: {
    fontSize: 48,
    marginBottom: 16,
  },
  emptyTitle: {
    fontSize: 18,
    fontWeight: '600',
    color: '#374151',
    marginBottom: 4,
  },
  emptyText: {
    fontSize: 14,
    color: '#6b7280',
  },
  faultCard: {
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 16,
    marginBottom: 12,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 1 },
    shadowOpacity: 0.05,
    shadowRadius: 2,
    elevation: 1,
  },
  faultHeader: {
    flexDirection: 'row',
    alignItems: 'flex-start',
    marginBottom: 8,
  },
  categoryIcon: {
    fontSize: 24,
    marginRight: 12,
  },
  faultTitleContainer: {
    flex: 1,
  },
  faultTitle: {
    fontSize: 16,
    fontWeight: '600',
    color: '#1f2937',
  },
  faultLocation: {
    fontSize: 13,
    color: '#6b7280',
    marginTop: 2,
  },
  statusBadge: {
    paddingHorizontal: 8,
    paddingVertical: 4,
    borderRadius: 4,
    marginLeft: 8,
  },
  statusText: {
    fontSize: 11,
    fontWeight: '600',
    color: '#fff',
    textTransform: 'uppercase',
  },
  faultDescription: {
    fontSize: 14,
    color: '#4b5563',
    lineHeight: 20,
    marginBottom: 12,
    marginLeft: 36,
  },
  faultFooter: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginLeft: 36,
  },
  priorityBadge: {
    paddingHorizontal: 8,
    paddingVertical: 2,
    borderRadius: 4,
    borderWidth: 1,
  },
  priorityText: {
    fontSize: 11,
    fontWeight: '600',
    textTransform: 'uppercase',
  },
  faultDate: {
    fontSize: 12,
    color: '#9ca3af',
  },
  assignedRow: {
    flexDirection: 'row',
    marginTop: 8,
    marginLeft: 36,
    paddingTop: 8,
    borderTopWidth: 1,
    borderTopColor: '#f3f4f6',
  },
  assignedLabel: {
    fontSize: 12,
    color: '#6b7280',
  },
  assignedValue: {
    fontSize: 12,
    color: '#2563eb',
    marginLeft: 4,
    fontWeight: '500',
  },
  bottomSpacer: {
    height: 100,
  },
});
