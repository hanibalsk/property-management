import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import Constants from 'expo-constants';
import { StatusBar } from 'expo-status-bar';
import { useCallback, useState } from 'react';
import { Pressable, StyleSheet, Text, View } from 'react-native';
import { AuthProvider, useAuth } from './contexts';
import { useOfflineSupport } from './hooks';
import {
  AnnouncementsScreen,
  DashboardScreen,
  DocumentsScreen,
  FaultsListScreen,
  LoginScreen,
  ReportFaultScreen,
  VotingScreen,
} from './screens';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 1000 * 60 * 5, // 5 minutes
      gcTime: 1000 * 60 * 30, // 30 minutes (formerly cacheTime)
      retry: 3,
    },
  },
});

const API_BASE_URL = (Constants.expoConfig?.extra?.apiUrl as string) || 'http://localhost:8080';

type Screen =
  | 'Dashboard'
  | 'Faults'
  | 'ReportFault'
  | 'Announcements'
  | 'Voting'
  | 'Documents'
  | 'Messages'
  | 'Settings';

function MainApp() {
  const { isAuthenticated, isLoading } = useAuth();
  const { isConnected, queuedActionsCount } = useOfflineSupport();
  const [currentScreen, setCurrentScreen] = useState<Screen>('Dashboard');

  const handleNavigate = useCallback((screen: string) => {
    setCurrentScreen(screen as Screen);
  }, []);

  if (isLoading) {
    return (
      <View style={styles.loadingContainer}>
        <Text style={styles.loadingText}>Loading...</Text>
      </View>
    );
  }

  if (!isAuthenticated) {
    return <LoginScreen />;
  }

  const renderScreen = () => {
    switch (currentScreen) {
      case 'Dashboard':
        return <DashboardScreen onNavigate={handleNavigate} />;
      case 'Faults':
        return <FaultsListScreen onNavigate={handleNavigate} />;
      case 'ReportFault':
        return (
          <ReportFaultScreen
            onSuccess={() => handleNavigate('Faults')}
            onCancel={() => handleNavigate('Faults')}
          />
        );
      case 'Announcements':
        return <AnnouncementsScreen onNavigate={handleNavigate} />;
      case 'Voting':
        return <VotingScreen onNavigate={handleNavigate} />;
      case 'Documents':
        return <DocumentsScreen onNavigate={handleNavigate} />;
      default:
        return <DashboardScreen onNavigate={handleNavigate} />;
    }
  };

  return (
    <View style={styles.container}>
      {/* Offline indicator */}
      {!isConnected && (
        <View style={styles.offlineBar}>
          <Text style={styles.offlineText}>
            Offline {queuedActionsCount > 0 && `(${queuedActionsCount} pending)`}
          </Text>
        </View>
      )}

      {/* Main content */}
      <View style={styles.content}>{renderScreen()}</View>

      {/* Bottom navigation */}
      <View style={styles.bottomNav}>
        <NavButton
          icon="🏠"
          label="Home"
          isActive={currentScreen === 'Dashboard'}
          onPress={() => handleNavigate('Dashboard')}
        />
        <NavButton
          icon="🔧"
          label="Faults"
          isActive={currentScreen === 'Faults' || currentScreen === 'ReportFault'}
          onPress={() => handleNavigate('Faults')}
        />
        <NavButton
          icon="📢"
          label="News"
          isActive={currentScreen === 'Announcements'}
          onPress={() => handleNavigate('Announcements')}
        />
        <NavButton
          icon="🗳️"
          label="Vote"
          isActive={currentScreen === 'Voting'}
          onPress={() => handleNavigate('Voting')}
        />
        <NavButton
          icon="📄"
          label="Docs"
          isActive={currentScreen === 'Documents'}
          onPress={() => handleNavigate('Documents')}
        />
      </View>

      <StatusBar style="auto" />
    </View>
  );
}

interface NavButtonProps {
  icon: string;
  label: string;
  isActive: boolean;
  onPress: () => void;
  badge?: number;
}

function NavButton({ icon, label, isActive, onPress, badge }: NavButtonProps) {
  return (
    <Pressable style={styles.navButton} onPress={onPress}>
      <View style={styles.navIconContainer}>
        <Text style={styles.navIcon}>{icon}</Text>
        {badge && badge > 0 && (
          <View style={styles.badge}>
            <Text style={styles.badgeText}>{badge > 99 ? '99+' : badge}</Text>
          </View>
        )}
      </View>
      <Text style={[styles.navLabel, isActive && styles.navLabelActive]}>{label}</Text>
    </Pressable>
  );
}

export default function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <AuthProvider apiBaseUrl={API_BASE_URL}>
        <MainApp />
      </AuthProvider>
    </QueryClientProvider>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  loadingContainer: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
  },
  loadingText: {
    fontSize: 16,
    color: '#6b7280',
  },
  offlineBar: {
    backgroundColor: '#fef2f2',
    paddingVertical: 8,
    paddingHorizontal: 16,
    paddingTop: 50,
  },
  offlineText: {
    color: '#dc2626',
    fontSize: 13,
    fontWeight: '500',
    textAlign: 'center',
  },
  content: {
    flex: 1,
  },
  bottomNav: {
    flexDirection: 'row',
    backgroundColor: '#fff',
    borderTopWidth: 1,
    borderTopColor: '#e5e7eb',
    paddingBottom: 24,
    paddingTop: 8,
  },
  navButton: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
    paddingVertical: 8,
  },
  navIconContainer: {
    position: 'relative',
  },
  navIcon: {
    fontSize: 24,
  },
  badge: {
    position: 'absolute',
    top: -4,
    right: -10,
    backgroundColor: '#ef4444',
    borderRadius: 10,
    minWidth: 18,
    height: 18,
    alignItems: 'center',
    justifyContent: 'center',
    paddingHorizontal: 4,
  },
  badgeText: {
    color: '#fff',
    fontSize: 10,
    fontWeight: 'bold',
  },
  navLabel: {
    fontSize: 11,
    color: '#6b7280',
    marginTop: 4,
  },
  navLabelActive: {
    color: '#2563eb',
    fontWeight: '600',
  },
});
