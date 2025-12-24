/**
 * Help center for contextual help, FAQs, and tutorials.
 *
 * Epic 50 - Story 50.2: Contextual Help
 * Epic 50 - Story 50.3: FAQ & Tutorials
 */
import AsyncStorage from '@react-native-async-storage/async-storage';

import type { FAQCategory, FAQItem, HelpContent, Tutorial } from './types';

const FAQ_VOTES_KEY = '@ppt/faq_votes';

/**
 * Provides help content, FAQs, and tutorials.
 */
export class HelpCenter {
  private helpContent: Map<string, HelpContent> = new Map();
  private faqs: Map<string, FAQItem> = new Map();
  private tutorials: Map<string, Tutorial> = new Map();
  private userVotes: Record<string, 'helpful' | 'not_helpful'> = {};

  constructor() {
    this.loadDefaultContent();
  }

  /**
   * Initialize and load user preferences.
   */
  async initialize(): Promise<void> {
    await this.loadUserVotes();
  }

  /**
   * Get help content for a screen.
   */
  getHelpForScreen(screenId: string): HelpContent | undefined {
    return this.helpContent.get(screenId);
  }

  /**
   * Search FAQs.
   */
  searchFAQs(query: string, category?: FAQCategory): FAQItem[] {
    const normalizedQuery = query.toLowerCase().trim();
    const results: Array<{ item: FAQItem; score: number }> = [];

    for (const faq of this.faqs.values()) {
      // Filter by category if specified
      if (category && faq.category !== category) {
        continue;
      }

      // Calculate relevance score
      let score = 0;

      if (faq.question.toLowerCase().includes(normalizedQuery)) {
        score += 10;
      }
      if (faq.answer.toLowerCase().includes(normalizedQuery)) {
        score += 5;
      }
      for (const tag of faq.tags) {
        if (tag.toLowerCase().includes(normalizedQuery)) {
          score += 3;
        }
      }

      if (score > 0 || !normalizedQuery) {
        results.push({ item: faq, score: score || faq.helpful - faq.notHelpful });
      }
    }

    // Sort by score descending
    results.sort((a, b) => b.score - a.score);

    return results.map((r) => r.item);
  }

  /**
   * Get FAQs by category.
   */
  getFAQsByCategory(category: FAQCategory): FAQItem[] {
    return Array.from(this.faqs.values()).filter((faq) => faq.category === category);
  }

  /**
   * Get all FAQ categories with counts.
   */
  getFAQCategories(): Array<{ category: FAQCategory; count: number; label: string }> {
    const counts: Partial<Record<FAQCategory, number>> = {};

    for (const faq of this.faqs.values()) {
      counts[faq.category] = (counts[faq.category] ?? 0) + 1;
    }

    const categoryLabels: Record<FAQCategory, string> = {
      getting_started: 'Getting Started',
      account: 'Account & Profile',
      buildings: 'Buildings',
      faults: 'Faults & Maintenance',
      voting: 'Voting',
      documents: 'Documents',
      payments: 'Payments',
      notifications: 'Notifications',
      security: 'Security & Privacy',
      other: 'Other',
    };

    return Object.entries(counts).map(([category, count]) => ({
      category: category as FAQCategory,
      count: count ?? 0,
      label: categoryLabels[category as FAQCategory],
    }));
  }

  /**
   * Vote on FAQ helpfulness.
   */
  async voteFAQ(faqId: string, helpful: boolean): Promise<void> {
    const faq = this.faqs.get(faqId);
    if (!faq) return;

    const previousVote = this.userVotes[faqId];

    // Remove previous vote
    if (previousVote === 'helpful') {
      faq.helpful -= 1;
    } else if (previousVote === 'not_helpful') {
      faq.notHelpful -= 1;
    }

    // Add new vote
    if (helpful) {
      faq.helpful += 1;
      this.userVotes[faqId] = 'helpful';
    } else {
      faq.notHelpful += 1;
      this.userVotes[faqId] = 'not_helpful';
    }

    await this.saveUserVotes();
  }

  /**
   * Get user's vote on FAQ.
   */
  getUserVote(faqId: string): 'helpful' | 'not_helpful' | null {
    return this.userVotes[faqId] ?? null;
  }

  /**
   * Get all tutorials.
   */
  getAllTutorials(): Tutorial[] {
    return Array.from(this.tutorials.values());
  }

  /**
   * Get tutorials by category.
   */
  getTutorialsByCategory(category: FAQCategory): Tutorial[] {
    return Array.from(this.tutorials.values()).filter((t) => t.category === category);
  }

  /**
   * Get a tutorial by ID.
   */
  getTutorial(id: string): Tutorial | undefined {
    return this.tutorials.get(id);
  }

  /**
   * Search tutorials.
   */
  searchTutorials(query: string): Tutorial[] {
    const normalizedQuery = query.toLowerCase().trim();

    return Array.from(this.tutorials.values()).filter(
      (t) =>
        t.title.toLowerCase().includes(normalizedQuery) ||
        t.description.toLowerCase().includes(normalizedQuery)
    );
  }

  // Private methods

  private async loadUserVotes(): Promise<void> {
    const stored = await AsyncStorage.getItem(FAQ_VOTES_KEY);
    if (stored) {
      this.userVotes = JSON.parse(stored);
    }
  }

  private async saveUserVotes(): Promise<void> {
    await AsyncStorage.setItem(FAQ_VOTES_KEY, JSON.stringify(this.userVotes));
  }

  private loadDefaultContent(): void {
    // Help content for screens
    const helpContents: HelpContent[] = [
      {
        id: 'dashboard',
        screen: 'Dashboard',
        title: 'Dashboard Help',
        shortDescription: 'Your overview of all building activities',
        fullContent:
          'The dashboard shows a summary of your buildings, recent notifications, and quick actions. You can see pending votes, active faults, and upcoming meetings at a glance.',
        relatedFAQs: ['faq-1', 'faq-2'],
        lastUpdated: '2025-01-01',
      },
      {
        id: 'buildings',
        screen: 'Buildings',
        title: 'Buildings Help',
        shortDescription: 'Manage your building portfolio',
        fullContent:
          'View all buildings you have access to. Tap on a building to see details, residents, faults, documents, and more.',
        relatedFAQs: ['faq-3'],
        lastUpdated: '2025-01-01',
      },
      {
        id: 'faults',
        screen: 'Faults',
        title: 'Fault Reporting Help',
        shortDescription: 'Report and track building issues',
        fullContent:
          'Report maintenance issues, track their status, and receive updates. Include photos and detailed descriptions for faster resolution.',
        relatedFAQs: ['faq-4', 'faq-5'],
        relatedTutorials: ['tutorial-faults'],
        lastUpdated: '2025-01-01',
      },
      {
        id: 'voting',
        screen: 'Voting',
        title: 'Voting Help',
        shortDescription: 'Participate in building decisions',
        fullContent:
          'Cast your vote on building proposals. View active votes, past results, and upcoming decisions.',
        relatedFAQs: ['faq-6'],
        relatedTutorials: ['tutorial-voting'],
        lastUpdated: '2025-01-01',
      },
      {
        id: 'documents',
        screen: 'Documents',
        title: 'Documents Help',
        shortDescription: 'Access building documents',
        fullContent:
          'View and download building documents including regulations, meeting minutes, contracts, and more.',
        relatedFAQs: ['faq-7'],
        lastUpdated: '2025-01-01',
      },
    ];

    for (const content of helpContents) {
      this.helpContent.set(content.screen, content);
    }

    // FAQs
    const faqItems: FAQItem[] = [
      {
        id: 'faq-1',
        question: 'How do I view my buildings?',
        answer:
          'Your buildings are displayed on the Dashboard. Tap on any building card to see its details, including residents, faults, documents, and more.',
        category: 'getting_started',
        tags: ['buildings', 'dashboard', 'view'],
        helpful: 42,
        notHelpful: 3,
      },
      {
        id: 'faq-2',
        question: 'How do I receive notifications?',
        answer:
          "Notifications are enabled by default. You can customize which notifications you receive in Settings > Notifications. Make sure you've allowed notifications for the app in your device settings.",
        category: 'notifications',
        tags: ['notifications', 'settings', 'alerts'],
        helpful: 35,
        notHelpful: 5,
      },
      {
        id: 'faq-3',
        question: 'How do I switch between buildings?',
        answer:
          'From the Dashboard or Buildings screen, tap on a different building card to switch. You can also use the building selector in the top menu.',
        category: 'buildings',
        tags: ['buildings', 'switch', 'select'],
        helpful: 28,
        notHelpful: 2,
      },
      {
        id: 'faq-4',
        question: 'How do I report a fault?',
        answer:
          "Tap the '+' button on the Dashboard or go to Faults > Report New. Select a category, describe the issue, add photos if needed, and submit. You'll receive updates as the fault is processed.",
        category: 'faults',
        tags: ['faults', 'report', 'maintenance'],
        helpful: 56,
        notHelpful: 4,
        relatedQuestions: ['faq-5'],
      },
      {
        id: 'faq-5',
        question: 'How do I track my fault report?',
        answer:
          'Go to Faults to see all your reports. Each report shows its current status (Open, In Progress, Resolved, etc.). Tap on a report to see details and updates from the maintenance team.',
        category: 'faults',
        tags: ['faults', 'track', 'status'],
        helpful: 38,
        notHelpful: 2,
        relatedQuestions: ['faq-4'],
      },
      {
        id: 'faq-6',
        question: 'How do I vote on a proposal?',
        answer:
          'Go to Voting to see active votes. Tap on a vote to read the proposal, then select your choice (For, Against, or Abstain). Your vote is anonymous and can be changed until voting closes.',
        category: 'voting',
        tags: ['voting', 'vote', 'proposal'],
        helpful: 45,
        notHelpful: 3,
      },
      {
        id: 'faq-7',
        question: 'How do I download documents?',
        answer:
          'Go to Documents, find the document you need, and tap the download icon. Documents are saved to your device and can be viewed offline.',
        category: 'documents',
        tags: ['documents', 'download', 'files'],
        helpful: 32,
        notHelpful: 4,
      },
      {
        id: 'faq-8',
        question: 'How do I change my password?',
        answer:
          "Go to Settings > Account > Change Password. You'll need to enter your current password and then your new password twice. Make sure to use a strong password.",
        category: 'account',
        tags: ['password', 'account', 'security'],
        helpful: 29,
        notHelpful: 1,
      },
      {
        id: 'faq-9',
        question: 'Is my data secure?',
        answer:
          'Yes. All data is encrypted in transit and at rest. We use industry-standard security practices including two-factor authentication, secure token storage, and regular security audits.',
        category: 'security',
        tags: ['security', 'privacy', 'data'],
        helpful: 51,
        notHelpful: 2,
      },
      {
        id: 'faq-10',
        question: 'How do I enable two-factor authentication?',
        answer:
          'Go to Settings > Security > Two-Factor Authentication. Scan the QR code with an authenticator app (like Google Authenticator or Authy) and enter the verification code.',
        category: 'security',
        tags: ['2fa', 'security', 'authentication'],
        helpful: 37,
        notHelpful: 3,
      },
    ];

    for (const faq of faqItems) {
      this.faqs.set(faq.id, faq);
    }

    // Tutorials
    const tutorialItems: Tutorial[] = [
      {
        id: 'tutorial-getting-started',
        title: 'Getting Started with PPT',
        description: 'Learn the basics of navigating and using the app.',
        duration: 180,
        category: 'getting_started',
        steps: [
          {
            id: 'step-1',
            title: 'Dashboard Overview',
            content:
              'The dashboard is your home screen. It shows your buildings, recent activity, and quick actions.',
          },
          {
            id: 'step-2',
            title: 'Navigating the App',
            content:
              'Use the bottom navigation to switch between Dashboard, Buildings, Faults, Documents, and More.',
          },
          {
            id: 'step-3',
            title: 'Notifications',
            content:
              'The bell icon shows your notifications. Tap it to see what needs your attention.',
          },
        ],
      },
      {
        id: 'tutorial-faults',
        title: 'How to Report and Track Faults',
        description: 'Complete guide to the fault reporting system.',
        duration: 240,
        category: 'faults',
        steps: [
          {
            id: 'step-1',
            title: 'Create a Report',
            content:
              "Tap the '+' button to start a new fault report. Select the appropriate category.",
          },
          {
            id: 'step-2',
            title: 'Add Details',
            content:
              'Describe the problem in detail. Include location, severity, and when it started.',
          },
          {
            id: 'step-3',
            title: 'Attach Photos',
            content: 'Add photos to help technicians understand the issue.',
          },
          {
            id: 'step-4',
            title: 'Track Progress',
            content: "After submitting, track your report's status in the Faults section.",
          },
        ],
      },
      {
        id: 'tutorial-voting',
        title: 'Participating in Building Votes',
        description: 'How to cast your vote and view results.',
        duration: 120,
        category: 'voting',
        steps: [
          {
            id: 'step-1',
            title: 'Find Active Votes',
            content: 'Go to the Voting section to see all active votes for your buildings.',
          },
          {
            id: 'step-2',
            title: 'Read the Proposal',
            content: 'Tap on a vote to read the full proposal and any supporting documents.',
          },
          {
            id: 'step-3',
            title: 'Cast Your Vote',
            content: 'Select your choice and confirm. Your vote is anonymous.',
          },
        ],
      },
    ];

    for (const tutorial of tutorialItems) {
      this.tutorials.set(tutorial.id, tutorial);
    }
  }
}

// Singleton instance
export const helpCenter = new HelpCenter();
