/**
 * Help center screen with FAQs and tutorials.
 *
 * Epic 50 - Story 50.2-50.3: Contextual Help, FAQ & Tutorials
 */
import { useCallback, useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Pressable, ScrollView, StyleSheet, Text, TextInput, View } from 'react-native';

import { helpCenter } from '../../onboarding';
import type { FAQCategory, FAQItem, Tutorial } from '../../onboarding/types';

interface HelpCenterScreenProps {
  onNavigate: (screen: string, params?: Record<string, unknown>) => void;
}

export function HelpCenterScreen({ onNavigate }: HelpCenterScreenProps) {
  const { t } = useTranslation();
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedCategory, setSelectedCategory] = useState<FAQCategory | null>(null);
  const [activeTab, setActiveTab] = useState<'faq' | 'tutorials'>('faq');
  const [expandedFAQ, setExpandedFAQ] = useState<string | null>(null);

  const categories = useMemo(() => helpCenter.getFAQCategories(), []);

  const filteredFAQs = useMemo(() => {
    return helpCenter.searchFAQs(searchQuery, selectedCategory ?? undefined);
  }, [searchQuery, selectedCategory]);

  const filteredTutorials = useMemo(() => {
    if (searchQuery) {
      return helpCenter.searchTutorials(searchQuery);
    }
    if (selectedCategory) {
      return helpCenter.getTutorialsByCategory(selectedCategory);
    }
    return helpCenter.getAllTutorials();
  }, [searchQuery, selectedCategory]);

  const handleFAQPress = useCallback((faqId: string) => {
    setExpandedFAQ((prev) => (prev === faqId ? null : faqId));
  }, []);

  const handleVote = useCallback(async (faqId: string, helpful: boolean) => {
    await helpCenter.voteFAQ(faqId, helpful);
  }, []);

  const handleTutorialPress = useCallback(
    (tutorial: Tutorial) => {
      onNavigate('TutorialDetail', { tutorialId: tutorial.id });
    },
    [onNavigate]
  );

  const renderFAQItem = useCallback(
    (faq: FAQItem) => {
      const isExpanded = expandedFAQ === faq.id;
      const userVote = helpCenter.getUserVote(faq.id);

      return (
        <View key={faq.id} style={styles.faqItem}>
          <Pressable style={styles.faqQuestion} onPress={() => handleFAQPress(faq.id)}>
            <Text style={styles.faqQuestionText}>{faq.question}</Text>
            <Text style={styles.expandIcon}>{isExpanded ? '−' : '+'}</Text>
          </Pressable>

          {isExpanded && (
            <View style={styles.faqAnswer}>
              <Text style={styles.faqAnswerText}>{faq.answer}</Text>

              <View style={styles.faqFeedback}>
                <Text style={styles.feedbackLabel}>{t('help.wasHelpful')}</Text>
                <View style={styles.feedbackButtons}>
                  <Pressable
                    style={[styles.feedbackButton, userVote === 'helpful' && styles.feedbackActive]}
                    onPress={() => handleVote(faq.id, true)}
                  >
                    <Text
                      style={[
                        styles.feedbackButtonText,
                        userVote === 'helpful' && styles.feedbackActiveText,
                      ]}
                    >
                      👍 {faq.helpful}
                    </Text>
                  </Pressable>
                  <Pressable
                    style={[
                      styles.feedbackButton,
                      userVote === 'not_helpful' && styles.feedbackActive,
                    ]}
                    onPress={() => handleVote(faq.id, false)}
                  >
                    <Text
                      style={[
                        styles.feedbackButtonText,
                        userVote === 'not_helpful' && styles.feedbackActiveText,
                      ]}
                    >
                      👎 {faq.notHelpful}
                    </Text>
                  </Pressable>
                </View>
              </View>
            </View>
          )}
        </View>
      );
    },
    [expandedFAQ, handleFAQPress, handleVote, t]
  );

  const renderTutorialItem = useCallback(
    (tutorial: Tutorial) => {
      const durationMin = Math.ceil(tutorial.duration / 60);

      return (
        <Pressable
          key={tutorial.id}
          style={styles.tutorialItem}
          onPress={() => handleTutorialPress(tutorial)}
        >
          <View style={styles.tutorialThumbnail}>
            <Text style={styles.playIcon}>▶</Text>
          </View>
          <View style={styles.tutorialInfo}>
            <Text style={styles.tutorialTitle}>{tutorial.title}</Text>
            <Text style={styles.tutorialDescription} numberOfLines={2}>
              {tutorial.description}
            </Text>
            <Text style={styles.tutorialDuration}>{durationMin} min</Text>
          </View>
        </Pressable>
      );
    },
    [handleTutorialPress]
  );

  return (
    <View style={styles.container}>
      <View style={styles.header}>
        <Pressable onPress={() => onNavigate('Dashboard')} style={styles.backButton}>
          <Text style={styles.backButtonText}>← {t('common.back')}</Text>
        </Pressable>
        <Text style={styles.title}>{t('help.title')}</Text>
      </View>

      <View style={styles.searchContainer}>
        <TextInput
          style={styles.searchInput}
          placeholder={t('help.searchPlaceholder')}
          value={searchQuery}
          onChangeText={setSearchQuery}
          placeholderTextColor="#9ca3af"
        />
      </View>

      <View style={styles.tabs}>
        <Pressable
          style={[styles.tab, activeTab === 'faq' && styles.tabActive]}
          onPress={() => setActiveTab('faq')}
        >
          <Text style={[styles.tabText, activeTab === 'faq' && styles.tabTextActive]}>
            {t('help.faqTab')}
          </Text>
        </Pressable>
        <Pressable
          style={[styles.tab, activeTab === 'tutorials' && styles.tabActive]}
          onPress={() => setActiveTab('tutorials')}
        >
          <Text style={[styles.tabText, activeTab === 'tutorials' && styles.tabTextActive]}>
            {t('help.tutorialsTab')}
          </Text>
        </Pressable>
      </View>

      <ScrollView
        horizontal
        showsHorizontalScrollIndicator={false}
        style={styles.categoriesContainer}
        contentContainerStyle={styles.categoriesContent}
      >
        <Pressable
          style={[styles.categoryChip, selectedCategory === null && styles.categoryChipActive]}
          onPress={() => setSelectedCategory(null)}
        >
          <Text
            style={[
              styles.categoryChipText,
              selectedCategory === null && styles.categoryChipTextActive,
            ]}
          >
            All
          </Text>
        </Pressable>
        {categories.map((cat) => (
          <Pressable
            key={cat.category}
            style={[
              styles.categoryChip,
              selectedCategory === cat.category && styles.categoryChipActive,
            ]}
            onPress={() => setSelectedCategory(cat.category)}
          >
            <Text
              style={[
                styles.categoryChipText,
                selectedCategory === cat.category && styles.categoryChipTextActive,
              ]}
            >
              {cat.label} ({cat.count})
            </Text>
          </Pressable>
        ))}
      </ScrollView>

      <ScrollView style={styles.content}>
        {activeTab === 'faq' ? (
          <>
            {filteredFAQs.length === 0 ? (
              <View style={styles.emptyState}>
                <Text style={styles.emptyIcon}>🔍</Text>
                <Text style={styles.emptyTitle}>{t('help.noFAQs')}</Text>
                <Text style={styles.emptyText}>{t('help.noFAQsMessage')}</Text>
              </View>
            ) : (
              filteredFAQs.map(renderFAQItem)
            )}
          </>
        ) : (
          <>
            {filteredTutorials.length === 0 ? (
              <View style={styles.emptyState}>
                <Text style={styles.emptyIcon}>🎬</Text>
                <Text style={styles.emptyTitle}>{t('help.noTutorials')}</Text>
                <Text style={styles.emptyText}>{t('help.noTutorialsMessage')}</Text>
              </View>
            ) : (
              filteredTutorials.map(renderTutorialItem)
            )}
          </>
        )}
      </ScrollView>

      <Pressable style={styles.feedbackFab} onPress={() => onNavigate('Feedback')}>
        <Text style={styles.feedbackFabText}>💬 {t('help.sendFeedbackButton')}</Text>
      </Pressable>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  header: {
    backgroundColor: '#fff',
    paddingTop: 60,
    paddingHorizontal: 16,
    paddingBottom: 16,
    borderBottomWidth: 1,
    borderBottomColor: '#e5e7eb',
  },
  backButton: {
    marginBottom: 8,
  },
  backButtonText: {
    color: '#2563eb',
    fontSize: 16,
  },
  title: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#111827',
  },
  searchContainer: {
    backgroundColor: '#fff',
    paddingHorizontal: 16,
    paddingVertical: 12,
    borderBottomWidth: 1,
    borderBottomColor: '#e5e7eb',
  },
  searchInput: {
    backgroundColor: '#f3f4f6',
    borderRadius: 8,
    paddingHorizontal: 16,
    paddingVertical: 12,
    fontSize: 16,
    color: '#111827',
  },
  tabs: {
    flexDirection: 'row',
    backgroundColor: '#fff',
    borderBottomWidth: 1,
    borderBottomColor: '#e5e7eb',
  },
  tab: {
    flex: 1,
    paddingVertical: 14,
    alignItems: 'center',
    borderBottomWidth: 2,
    borderBottomColor: 'transparent',
  },
  tabActive: {
    borderBottomColor: '#2563eb',
  },
  tabText: {
    fontSize: 16,
    fontWeight: '500',
    color: '#6b7280',
  },
  tabTextActive: {
    color: '#2563eb',
  },
  categoriesContainer: {
    backgroundColor: '#fff',
    maxHeight: 56,
    borderBottomWidth: 1,
    borderBottomColor: '#e5e7eb',
  },
  categoriesContent: {
    paddingHorizontal: 12,
    paddingVertical: 10,
    gap: 8,
  },
  categoryChip: {
    paddingHorizontal: 14,
    paddingVertical: 8,
    backgroundColor: '#f3f4f6',
    borderRadius: 20,
    marginRight: 8,
  },
  categoryChipActive: {
    backgroundColor: '#2563eb',
  },
  categoryChipText: {
    fontSize: 14,
    color: '#6b7280',
  },
  categoryChipTextActive: {
    color: '#fff',
  },
  content: {
    flex: 1,
    padding: 16,
  },
  faqItem: {
    backgroundColor: '#fff',
    borderRadius: 12,
    marginBottom: 12,
    overflow: 'hidden',
  },
  faqQuestion: {
    flexDirection: 'row',
    alignItems: 'center',
    padding: 16,
  },
  faqQuestionText: {
    flex: 1,
    fontSize: 16,
    fontWeight: '500',
    color: '#111827',
  },
  expandIcon: {
    fontSize: 20,
    color: '#6b7280',
    marginLeft: 12,
  },
  faqAnswer: {
    padding: 16,
    paddingTop: 0,
    borderTopWidth: 1,
    borderTopColor: '#f3f4f6',
  },
  faqAnswerText: {
    fontSize: 15,
    color: '#4b5563',
    lineHeight: 22,
  },
  faqFeedback: {
    flexDirection: 'row',
    alignItems: 'center',
    marginTop: 16,
    paddingTop: 12,
    borderTopWidth: 1,
    borderTopColor: '#f3f4f6',
  },
  feedbackLabel: {
    fontSize: 14,
    color: '#6b7280',
    marginRight: 12,
  },
  feedbackButtons: {
    flexDirection: 'row',
    gap: 8,
  },
  feedbackButton: {
    paddingHorizontal: 12,
    paddingVertical: 6,
    borderRadius: 6,
    backgroundColor: '#f3f4f6',
  },
  feedbackActive: {
    backgroundColor: '#dbeafe',
  },
  feedbackButtonText: {
    fontSize: 14,
    color: '#6b7280',
  },
  feedbackActiveText: {
    color: '#2563eb',
  },
  tutorialItem: {
    flexDirection: 'row',
    backgroundColor: '#fff',
    borderRadius: 12,
    marginBottom: 12,
    overflow: 'hidden',
  },
  tutorialThumbnail: {
    width: 100,
    height: 80,
    backgroundColor: '#1f2937',
    alignItems: 'center',
    justifyContent: 'center',
  },
  playIcon: {
    fontSize: 24,
    color: '#fff',
  },
  tutorialInfo: {
    flex: 1,
    padding: 12,
  },
  tutorialTitle: {
    fontSize: 16,
    fontWeight: '600',
    color: '#111827',
    marginBottom: 4,
  },
  tutorialDescription: {
    fontSize: 14,
    color: '#6b7280',
    marginBottom: 8,
  },
  tutorialDuration: {
    fontSize: 12,
    color: '#9ca3af',
  },
  emptyState: {
    alignItems: 'center',
    padding: 40,
  },
  emptyIcon: {
    fontSize: 48,
    marginBottom: 16,
  },
  emptyTitle: {
    fontSize: 18,
    fontWeight: '600',
    color: '#111827',
    marginBottom: 8,
  },
  emptyText: {
    fontSize: 14,
    color: '#6b7280',
    textAlign: 'center',
  },
  feedbackFab: {
    position: 'absolute',
    bottom: 24,
    right: 24,
    backgroundColor: '#2563eb',
    paddingHorizontal: 20,
    paddingVertical: 14,
    borderRadius: 28,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.25,
    shadowRadius: 4,
    elevation: 5,
  },
  feedbackFabText: {
    color: '#fff',
    fontSize: 16,
    fontWeight: '600',
  },
});
