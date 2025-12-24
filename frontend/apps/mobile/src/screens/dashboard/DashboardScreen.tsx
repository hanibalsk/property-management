import { useCallback, useState } from 'react';
import { Pressable, RefreshControl, ScrollView, StyleSheet, Text, View } from 'react-native';
import { useAuth } from '../../contexts/AuthContext';

// Dashboard card types
interface DashboardStats {
  pendingFaults: number;
  unreadAnnouncements: number;
  activeVotes: number;
  unreadMessages: number;
  upcomingPayments: number;
}

interface Announcement {
  id: string;
  title: string;
  createdAt: string;
  category: 'general' | 'urgent' | 'maintenance' | 'event';
}

interface PendingAction {
  id: string;
  type: 'vote' | 'payment' | 'reading' | 'fault';
  title: string;
  dueDate?: string;
}

// Mock data - would come from API
const mockStats: DashboardStats = {
  pendingFaults: 2,
  unreadAnnouncements: 3,
  activeVotes: 1,
  unreadMessages: 5,
  upcomingPayments: 1,
};

const mockAnnouncements: Announcement[] = [
  {
    id: '1',
    title: 'Water shutdown scheduled for maintenance',
    createdAt: '2025-12-24T10:00:00Z',
    category: 'maintenance',
  },
  {
    id: '2',
    title: 'Annual building meeting on January 15th',
    createdAt: '2025-12-23T14:00:00Z',
    category: 'event',
  },
  {
    id: '3',
    title: 'New recycling guidelines',
    createdAt: '2025-12-22T09:00:00Z',
    category: 'general',
  },
];

const mockPendingActions: PendingAction[] = [
  { id: '1', type: 'vote', title: 'Vote on elevator renovation', dueDate: '2025-12-30' },
  { id: '2', type: 'payment', title: 'Monthly fees due', dueDate: '2025-01-05' },
  { id: '3', type: 'reading', title: 'Submit water meter reading', dueDate: '2025-01-01' },
];

interface DashboardScreenProps {
  onNavigate?: (screen: string) => void;
}

export function DashboardScreen({ onNavigate }: DashboardScreenProps) {
  const { user, logout } = useAuth();
  const [refreshing, setRefreshing] = useState(false);
  const [stats] = useState<DashboardStats>(mockStats);
  const [announcements] = useState<Announcement[]>(mockAnnouncements);
  const [pendingActions] = useState<PendingAction[]>(mockPendingActions);

  const onRefresh = useCallback(async () => {
    setRefreshing(true);
    // Simulate API refresh
    await new Promise((resolve) => setTimeout(resolve, 1000));
    setRefreshing(false);
  }, []);

  const formatDate = (dateString: string): string => {
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
  };

  const getCategoryColor = (category: Announcement['category']): string => {
    switch (category) {
      case 'urgent':
        return '#ef4444';
      case 'maintenance':
        return '#f59e0b';
      case 'event':
        return '#8b5cf6';
      default:
        return '#6b7280';
    }
  };

  const getActionIcon = (type: PendingAction['type']): string => {
    switch (type) {
      case 'vote':
        return '🗳️';
      case 'payment':
        return '💳';
      case 'reading':
        return '📊';
      case 'fault':
        return '🔧';
      default:
        return '📋';
    }
  };

  return (
    <View style={styles.container}>
      {/* Header */}
      <View style={styles.header}>
        <View>
          <Text style={styles.greeting}>Welcome back,</Text>
          <Text style={styles.userName}>
            {user?.firstName} {user?.lastName}
          </Text>
        </View>
        <Pressable style={styles.logoutButton} onPress={logout}>
          <Text style={styles.logoutText}>Logout</Text>
        </Pressable>
      </View>

      <ScrollView
        style={styles.scrollView}
        refreshControl={
          <RefreshControl refreshing={refreshing} onRefresh={onRefresh} tintColor="#2563eb" />
        }
      >
        {/* Stats Grid */}
        <View style={styles.statsGrid}>
          <Pressable style={styles.statCard} onPress={() => onNavigate?.('Faults')}>
            <Text style={styles.statNumber}>{stats.pendingFaults}</Text>
            <Text style={styles.statLabel}>Open Faults</Text>
          </Pressable>
          <Pressable style={styles.statCard} onPress={() => onNavigate?.('Announcements')}>
            <Text style={styles.statNumber}>{stats.unreadAnnouncements}</Text>
            <Text style={styles.statLabel}>New Announcements</Text>
          </Pressable>
          <Pressable style={styles.statCard} onPress={() => onNavigate?.('Voting')}>
            <Text style={styles.statNumber}>{stats.activeVotes}</Text>
            <Text style={styles.statLabel}>Active Votes</Text>
          </Pressable>
          <Pressable style={styles.statCard} onPress={() => onNavigate?.('Messages')}>
            <Text style={styles.statNumber}>{stats.unreadMessages}</Text>
            <Text style={styles.statLabel}>Unread Messages</Text>
          </Pressable>
        </View>

        {/* Pending Actions */}
        {pendingActions.length > 0 && (
          <View style={styles.section}>
            <Text style={styles.sectionTitle}>Pending Actions</Text>
            <View style={styles.actionsList}>
              {pendingActions.map((action) => (
                <Pressable key={action.id} style={styles.actionCard}>
                  <Text style={styles.actionIcon}>{getActionIcon(action.type)}</Text>
                  <View style={styles.actionContent}>
                    <Text style={styles.actionTitle}>{action.title}</Text>
                    {action.dueDate && (
                      <Text style={styles.actionDue}>Due: {formatDate(action.dueDate)}</Text>
                    )}
                  </View>
                  <Text style={styles.actionArrow}>›</Text>
                </Pressable>
              ))}
            </View>
          </View>
        )}

        {/* Recent Announcements */}
        <View style={styles.section}>
          <View style={styles.sectionHeader}>
            <Text style={styles.sectionTitle}>Recent Announcements</Text>
            <Pressable onPress={() => onNavigate?.('Announcements')}>
              <Text style={styles.seeAllText}>See All</Text>
            </Pressable>
          </View>
          <View style={styles.announcementsList}>
            {announcements.map((announcement) => (
              <Pressable key={announcement.id} style={styles.announcementCard}>
                <View style={styles.announcementHeader}>
                  <View
                    style={[
                      styles.categoryBadge,
                      { backgroundColor: getCategoryColor(announcement.category) },
                    ]}
                  >
                    <Text style={styles.categoryText}>{announcement.category}</Text>
                  </View>
                  <Text style={styles.announcementDate}>{formatDate(announcement.createdAt)}</Text>
                </View>
                <Text style={styles.announcementTitle}>{announcement.title}</Text>
              </Pressable>
            ))}
          </View>
        </View>

        {/* Quick Actions */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Quick Actions</Text>
          <View style={styles.quickActionsGrid}>
            <Pressable style={styles.quickAction} onPress={() => onNavigate?.('ReportFault')}>
              <Text style={styles.quickActionIcon}>🔧</Text>
              <Text style={styles.quickActionLabel}>Report Fault</Text>
            </Pressable>
            <Pressable style={styles.quickAction} onPress={() => onNavigate?.('Documents')}>
              <Text style={styles.quickActionIcon}>📄</Text>
              <Text style={styles.quickActionLabel}>Documents</Text>
            </Pressable>
            <Pressable style={styles.quickAction} onPress={() => onNavigate?.('MeterReading')}>
              <Text style={styles.quickActionIcon}>📊</Text>
              <Text style={styles.quickActionLabel}>Meter Reading</Text>
            </Pressable>
            <Pressable style={styles.quickAction} onPress={() => onNavigate?.('Payments')}>
              <Text style={styles.quickActionIcon}>💳</Text>
              <Text style={styles.quickActionLabel}>Payments</Text>
            </Pressable>
          </View>
        </View>

        {/* Bottom spacing */}
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
    backgroundColor: '#2563eb',
  },
  greeting: {
    fontSize: 14,
    color: 'rgba(255, 255, 255, 0.8)',
  },
  userName: {
    fontSize: 20,
    fontWeight: '600',
    color: '#fff',
  },
  logoutButton: {
    padding: 8,
  },
  logoutText: {
    color: 'rgba(255, 255, 255, 0.9)',
    fontSize: 14,
  },
  scrollView: {
    flex: 1,
  },
  statsGrid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    padding: 16,
    gap: 12,
  },
  statCard: {
    flex: 1,
    minWidth: '45%',
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 16,
    alignItems: 'center',
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 1 },
    shadowOpacity: 0.05,
    shadowRadius: 2,
    elevation: 1,
  },
  statNumber: {
    fontSize: 28,
    fontWeight: 'bold',
    color: '#2563eb',
  },
  statLabel: {
    fontSize: 12,
    color: '#6b7280',
    marginTop: 4,
    textAlign: 'center',
  },
  section: {
    padding: 16,
  },
  sectionHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 12,
  },
  sectionTitle: {
    fontSize: 18,
    fontWeight: '600',
    color: '#1f2937',
    marginBottom: 12,
  },
  seeAllText: {
    color: '#2563eb',
    fontSize: 14,
  },
  actionsList: {
    gap: 8,
  },
  actionCard: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 16,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 1 },
    shadowOpacity: 0.05,
    shadowRadius: 2,
    elevation: 1,
  },
  actionIcon: {
    fontSize: 24,
    marginRight: 12,
  },
  actionContent: {
    flex: 1,
  },
  actionTitle: {
    fontSize: 15,
    fontWeight: '500',
    color: '#1f2937',
  },
  actionDue: {
    fontSize: 13,
    color: '#f59e0b',
    marginTop: 2,
  },
  actionArrow: {
    fontSize: 24,
    color: '#9ca3af',
  },
  announcementsList: {
    gap: 8,
  },
  announcementCard: {
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 16,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 1 },
    shadowOpacity: 0.05,
    shadowRadius: 2,
    elevation: 1,
  },
  announcementHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 8,
  },
  categoryBadge: {
    paddingHorizontal: 8,
    paddingVertical: 4,
    borderRadius: 4,
  },
  categoryText: {
    fontSize: 11,
    fontWeight: '600',
    color: '#fff',
    textTransform: 'uppercase',
  },
  announcementDate: {
    fontSize: 12,
    color: '#9ca3af',
  },
  announcementTitle: {
    fontSize: 15,
    color: '#1f2937',
    lineHeight: 20,
  },
  quickActionsGrid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    gap: 12,
  },
  quickAction: {
    flex: 1,
    minWidth: '45%',
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 20,
    alignItems: 'center',
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 1 },
    shadowOpacity: 0.05,
    shadowRadius: 2,
    elevation: 1,
  },
  quickActionIcon: {
    fontSize: 32,
    marginBottom: 8,
  },
  quickActionLabel: {
    fontSize: 13,
    color: '#374151',
    fontWeight: '500',
  },
  bottomSpacer: {
    height: 100,
  },
});
