import { useCallback, useState } from 'react';
import { Alert, Pressable, RefreshControl, ScrollView, StyleSheet, Text, View } from 'react-native';

export type VoteStatus = 'active' | 'closed' | 'pending';
export type VoteType = 'simple' | 'multiple' | 'weighted' | 'ranked';

export interface VoteOption {
  id: string;
  label: string;
  votes: number;
  percentage: number;
}

export interface Vote {
  id: string;
  title: string;
  description: string;
  status: VoteStatus;
  type: VoteType;
  options: VoteOption[];
  startsAt: string;
  endsAt: string;
  totalVotes: number;
  requiredQuorum: number;
  currentQuorum: number;
  hasVoted: boolean;
  userVote?: string | string[];
  createdBy: string;
}

// Mock data
const mockVotes: Vote[] = [
  {
    id: '1',
    title: 'Elevator Renovation Proposal',
    description:
      'Vote on the proposed elevator modernization project. The renovation includes new control systems, improved safety features, and energy-efficient motors.',
    status: 'active',
    type: 'simple',
    options: [
      { id: 'yes', label: 'Yes - Approve renovation', votes: 45, percentage: 60 },
      { id: 'no', label: 'No - Decline proposal', votes: 30, percentage: 40 },
    ],
    startsAt: '2025-12-20T00:00:00Z',
    endsAt: '2025-12-30T23:59:59Z',
    totalVotes: 75,
    requiredQuorum: 100,
    currentQuorum: 75,
    hasVoted: false,
    createdBy: 'Building Management',
  },
  {
    id: '2',
    title: 'New Board Member Election',
    description: 'Vote for the new board member position. Each owner can vote for one candidate.',
    status: 'active',
    type: 'simple',
    options: [
      { id: 'a', label: 'John Smith', votes: 35, percentage: 44 },
      { id: 'b', label: 'Maria Garcia', votes: 28, percentage: 35 },
      { id: 'c', label: 'Peter Johnson', votes: 17, percentage: 21 },
    ],
    startsAt: '2025-12-15T00:00:00Z',
    endsAt: '2025-12-28T23:59:59Z',
    totalVotes: 80,
    requiredQuorum: 100,
    currentQuorum: 80,
    hasVoted: true,
    userVote: 'a',
    createdBy: 'Building Management',
  },
  {
    id: '3',
    title: 'Parking Space Allocation',
    description: 'Closed vote on new parking space allocation policy.',
    status: 'closed',
    type: 'simple',
    options: [
      { id: 'rotate', label: 'Rotating system', votes: 65, percentage: 65 },
      { id: 'fixed', label: 'Fixed assignment', votes: 35, percentage: 35 },
    ],
    startsAt: '2025-12-01T00:00:00Z',
    endsAt: '2025-12-15T23:59:59Z',
    totalVotes: 100,
    requiredQuorum: 80,
    currentQuorum: 100,
    hasVoted: true,
    userVote: 'rotate',
    createdBy: 'Building Management',
  },
];

interface VotingScreenProps {
  onNavigate?: (screen: string, params?: Record<string, unknown>) => void;
}

export function VotingScreen({ onNavigate: _onNavigate }: VotingScreenProps) {
  const [refreshing, setRefreshing] = useState(false);
  const [votes, setVotes] = useState<Vote[]>(mockVotes);
  const [filter, setFilter] = useState<'all' | 'active' | 'closed'>('all');

  const onRefresh = useCallback(async () => {
    setRefreshing(true);
    await new Promise((resolve) => setTimeout(resolve, 1000));
    setRefreshing(false);
  }, []);

  const getStatusColor = (status: VoteStatus): string => {
    switch (status) {
      case 'active':
        return '#10b981';
      case 'closed':
        return '#6b7280';
      case 'pending':
        return '#f59e0b';
      default:
        return '#6b7280';
    }
  };

  const formatDate = (dateString: string): string => {
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
  };

  const getTimeRemaining = (endsAt: string): string => {
    const end = new Date(endsAt);
    const now = new Date();
    const diff = end.getTime() - now.getTime();

    if (diff <= 0) return 'Ended';

    const days = Math.floor(diff / (1000 * 60 * 60 * 24));
    const hours = Math.floor((diff % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));

    if (days > 0) return `${days}d ${hours}h left`;
    if (hours > 0) return `${hours}h left`;

    const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
    return `${minutes}m left`;
  };

  const handleVote = (voteId: string, optionId: string) => {
    Alert.alert(
      'Confirm Vote',
      'Are you sure you want to cast your vote? This action cannot be undone.',
      [
        { text: 'Cancel', style: 'cancel' },
        {
          text: 'Vote',
          onPress: () => {
            setVotes((prev) =>
              prev.map((v) => {
                if (v.id !== voteId) return v;
                return {
                  ...v,
                  hasVoted: true,
                  userVote: optionId,
                  options: v.options.map((o) => ({
                    ...o,
                    votes: o.id === optionId ? o.votes + 1 : o.votes,
                    percentage: Math.round(
                      ((o.id === optionId ? o.votes + 1 : o.votes) / (v.totalVotes + 1)) * 100
                    ),
                  })),
                  totalVotes: v.totalVotes + 1,
                  currentQuorum: v.currentQuorum + 1,
                };
              })
            );
            Alert.alert('Success', 'Your vote has been recorded.');
          },
        },
      ]
    );
  };

  const filteredVotes = votes.filter((v) => {
    if (filter === 'all') return true;
    return v.status === filter;
  });

  const activeCount = votes.filter((v) => v.status === 'active').length;

  return (
    <View style={styles.container}>
      {/* Header */}
      <View style={styles.header}>
        <Text style={styles.headerTitle}>Voting</Text>
        {activeCount > 0 && (
          <View style={styles.activeCountBadge}>
            <Text style={styles.activeCountText}>{activeCount} active</Text>
          </View>
        )}
      </View>

      {/* Filters */}
      <View style={styles.filters}>
        {(['all', 'active', 'closed'] as const).map((option) => (
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

      {/* Votes List */}
      <ScrollView
        style={styles.scrollView}
        refreshControl={
          <RefreshControl refreshing={refreshing} onRefresh={onRefresh} tintColor="#2563eb" />
        }
      >
        {filteredVotes.length === 0 ? (
          <View style={styles.emptyState}>
            <Text style={styles.emptyIcon}>🗳️</Text>
            <Text style={styles.emptyTitle}>No votes found</Text>
            <Text style={styles.emptyText}>Check back for future voting</Text>
          </View>
        ) : (
          filteredVotes.map((vote) => (
            <View key={vote.id} style={styles.voteCard}>
              <View style={styles.voteHeader}>
                <View
                  style={[styles.statusBadge, { backgroundColor: getStatusColor(vote.status) }]}
                >
                  <Text style={styles.statusText}>{vote.status}</Text>
                </View>
                {vote.status === 'active' && (
                  <Text style={styles.timeRemaining}>{getTimeRemaining(vote.endsAt)}</Text>
                )}
              </View>

              <Text style={styles.voteTitle}>{vote.title}</Text>
              <Text style={styles.voteDescription} numberOfLines={2}>
                {vote.description}
              </Text>

              {/* Quorum Progress */}
              <View style={styles.quorumSection}>
                <View style={styles.quorumHeader}>
                  <Text style={styles.quorumLabel}>Quorum</Text>
                  <Text style={styles.quorumValue}>
                    {vote.currentQuorum}/{vote.requiredQuorum} votes
                  </Text>
                </View>
                <View style={styles.quorumBar}>
                  <View
                    style={[
                      styles.quorumProgress,
                      {
                        width: `${Math.min((vote.currentQuorum / vote.requiredQuorum) * 100, 100)}%`,
                      },
                      vote.currentQuorum >= vote.requiredQuorum && styles.quorumMet,
                    ]}
                  />
                </View>
              </View>

              {/* Vote Options */}
              <View style={styles.optionsSection}>
                {vote.options.map((option) => (
                  <Pressable
                    key={option.id}
                    style={[
                      styles.optionRow,
                      vote.hasVoted && vote.userVote === option.id && styles.selectedOption,
                      vote.status !== 'active' && styles.optionDisabled,
                    ]}
                    onPress={() => {
                      if (!vote.hasVoted && vote.status === 'active') {
                        handleVote(vote.id, option.id);
                      }
                    }}
                    disabled={vote.hasVoted || vote.status !== 'active'}
                  >
                    <View style={styles.optionLeft}>
                      <View
                        style={[
                          styles.radioButton,
                          vote.hasVoted && vote.userVote === option.id && styles.radioSelected,
                        ]}
                      >
                        {vote.hasVoted && vote.userVote === option.id && (
                          <View style={styles.radioInner} />
                        )}
                      </View>
                      <Text
                        style={[
                          styles.optionLabel,
                          vote.hasVoted &&
                            vote.userVote === option.id &&
                            styles.optionLabelSelected,
                        ]}
                      >
                        {option.label}
                      </Text>
                    </View>
                    {(vote.hasVoted || vote.status === 'closed') && (
                      <View style={styles.optionRight}>
                        <Text style={styles.percentageText}>{option.percentage}%</Text>
                        <View style={styles.percentageBar}>
                          <View
                            style={[styles.percentageProgress, { width: `${option.percentage}%` }]}
                          />
                        </View>
                      </View>
                    )}
                  </Pressable>
                ))}
              </View>

              {/* Vote Status Message */}
              {vote.hasVoted && (
                <View style={styles.votedMessage}>
                  <Text style={styles.votedIcon}>✓</Text>
                  <Text style={styles.votedText}>You have voted</Text>
                </View>
              )}

              {/* Footer */}
              <View style={styles.voteFooter}>
                <Text style={styles.createdBy}>Created by {vote.createdBy}</Text>
                <Text style={styles.dateRange}>
                  {formatDate(vote.startsAt)} - {formatDate(vote.endsAt)}
                </Text>
              </View>
            </View>
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
  activeCountBadge: {
    backgroundColor: '#dcfce7',
    paddingHorizontal: 10,
    paddingVertical: 4,
    borderRadius: 12,
  },
  activeCountText: {
    color: '#16a34a',
    fontSize: 13,
    fontWeight: '600',
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
  voteCard: {
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 16,
    marginBottom: 16,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 1 },
    shadowOpacity: 0.05,
    shadowRadius: 2,
    elevation: 1,
  },
  voteHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 12,
  },
  statusBadge: {
    paddingHorizontal: 8,
    paddingVertical: 4,
    borderRadius: 4,
  },
  statusText: {
    fontSize: 11,
    fontWeight: '600',
    color: '#fff',
    textTransform: 'uppercase',
  },
  timeRemaining: {
    fontSize: 13,
    color: '#f59e0b',
    fontWeight: '600',
  },
  voteTitle: {
    fontSize: 17,
    fontWeight: '600',
    color: '#1f2937',
    marginBottom: 6,
  },
  voteDescription: {
    fontSize: 14,
    color: '#6b7280',
    lineHeight: 20,
    marginBottom: 16,
  },
  quorumSection: {
    marginBottom: 16,
  },
  quorumHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: 6,
  },
  quorumLabel: {
    fontSize: 12,
    color: '#6b7280',
    fontWeight: '500',
  },
  quorumValue: {
    fontSize: 12,
    color: '#374151',
    fontWeight: '500',
  },
  quorumBar: {
    height: 6,
    backgroundColor: '#e5e7eb',
    borderRadius: 3,
    overflow: 'hidden',
  },
  quorumProgress: {
    height: '100%',
    backgroundColor: '#f59e0b',
    borderRadius: 3,
  },
  quorumMet: {
    backgroundColor: '#10b981',
  },
  optionsSection: {
    gap: 8,
    marginBottom: 12,
  },
  optionRow: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    padding: 12,
    backgroundColor: '#f9fafb',
    borderRadius: 8,
    borderWidth: 1,
    borderColor: '#e5e7eb',
  },
  selectedOption: {
    backgroundColor: '#eff6ff',
    borderColor: '#2563eb',
  },
  optionDisabled: {
    opacity: 0.8,
  },
  optionLeft: {
    flexDirection: 'row',
    alignItems: 'center',
    flex: 1,
  },
  radioButton: {
    width: 20,
    height: 20,
    borderRadius: 10,
    borderWidth: 2,
    borderColor: '#d1d5db',
    alignItems: 'center',
    justifyContent: 'center',
    marginRight: 10,
  },
  radioSelected: {
    borderColor: '#2563eb',
  },
  radioInner: {
    width: 10,
    height: 10,
    borderRadius: 5,
    backgroundColor: '#2563eb',
  },
  optionLabel: {
    fontSize: 14,
    color: '#374151',
    flex: 1,
  },
  optionLabelSelected: {
    color: '#2563eb',
    fontWeight: '500',
  },
  optionRight: {
    alignItems: 'flex-end',
    minWidth: 80,
  },
  percentageText: {
    fontSize: 13,
    fontWeight: '600',
    color: '#374151',
    marginBottom: 4,
  },
  percentageBar: {
    height: 4,
    width: 60,
    backgroundColor: '#e5e7eb',
    borderRadius: 2,
    overflow: 'hidden',
  },
  percentageProgress: {
    height: '100%',
    backgroundColor: '#2563eb',
    borderRadius: 2,
  },
  votedMessage: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    padding: 8,
    backgroundColor: '#dcfce7',
    borderRadius: 6,
    marginTop: 4,
    marginBottom: 12,
    gap: 6,
  },
  votedIcon: {
    fontSize: 14,
    color: '#16a34a',
  },
  votedText: {
    fontSize: 13,
    color: '#16a34a',
    fontWeight: '500',
  },
  voteFooter: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    paddingTop: 12,
    borderTopWidth: 1,
    borderTopColor: '#f3f4f6',
  },
  createdBy: {
    fontSize: 12,
    color: '#9ca3af',
  },
  dateRange: {
    fontSize: 12,
    color: '#9ca3af',
  },
  bottomSpacer: {
    height: 100,
  },
});
