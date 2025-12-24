/**
 * Feedback and bug report screen.
 *
 * Epic 50 - Story 50.4: Feedback & Bug Reports
 */
import { useCallback, useMemo, useState } from 'react';
import {
  Alert,
  KeyboardAvoidingView,
  Platform,
  Pressable,
  ScrollView,
  StyleSheet,
  Text,
  TextInput,
  View,
} from 'react-native';

import { FeedbackManager } from '../../onboarding';
import type { FeedbackType } from '../../onboarding/types';

interface FeedbackScreenProps {
  onNavigate: (screen: string) => void;
}

const API_BASE_URL = 'http://localhost:8080';
const APP_VERSION = '1.0.0';
const BUILD_NUMBER = '1';

export function FeedbackScreen({ onNavigate }: FeedbackScreenProps) {
  const [feedbackType, setFeedbackType] = useState<FeedbackType>('bug');
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [email, setEmail] = useState('');
  const [includeDeviceInfo, setIncludeDeviceInfo] = useState(true);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const feedbackManager = useMemo(
    () => new FeedbackManager(API_BASE_URL, APP_VERSION, BUILD_NUMBER),
    []
  );

  const feedbackTypes = useMemo(() => feedbackManager.getFeedbackTypes(), [feedbackManager]);

  const deviceInfo = useMemo(() => feedbackManager.getDeviceInfo(), [feedbackManager]);

  const diagnosticReport = useMemo(
    () => feedbackManager.generateDiagnosticReport(),
    [feedbackManager]
  );

  const handleSubmit = useCallback(async () => {
    if (!title.trim()) {
      Alert.alert('Error', 'Please enter a title');
      return;
    }

    if (!description.trim()) {
      Alert.alert('Error', 'Please enter a description');
      return;
    }

    setIsSubmitting(true);

    const feedback = feedbackManager.createFeedback(
      feedbackType,
      title.trim(),
      description.trim(),
      email.trim() || undefined,
      undefined // screenshot would be captured here
    );

    const result = await feedbackManager.submitFeedback(feedback);

    setIsSubmitting(false);

    if (result.success) {
      Alert.alert('Thank You!', 'Your feedback has been submitted successfully.', [
        { text: 'OK', onPress: () => onNavigate('HelpCenter') },
      ]);
    } else {
      Alert.alert(
        'Saved for Later',
        'Your feedback has been saved and will be submitted when you have an internet connection.',
        [{ text: 'OK' }]
      );
    }
  }, [feedbackManager, feedbackType, title, description, email, onNavigate]);

  const handleSaveDraft = useCallback(async () => {
    if (!title.trim() && !description.trim()) {
      Alert.alert('Error', 'Nothing to save');
      return;
    }

    await feedbackManager.saveDraft({
      type: feedbackType,
      title: title.trim(),
      description: description.trim(),
      email: email.trim() || undefined,
    });

    Alert.alert('Draft Saved', 'Your feedback draft has been saved.');
  }, [feedbackManager, feedbackType, title, description, email]);

  return (
    <KeyboardAvoidingView
      style={styles.container}
      behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
    >
      <View style={styles.header}>
        <Pressable onPress={() => onNavigate('HelpCenter')} style={styles.backButton}>
          <Text style={styles.backButtonText}>← Back</Text>
        </Pressable>
        <Text style={styles.title}>Send Feedback</Text>
      </View>

      <ScrollView style={styles.content} keyboardShouldPersistTaps="handled">
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Feedback Type</Text>
          <View style={styles.typeGrid}>
            {feedbackTypes.map((type) => (
              <Pressable
                key={type.type}
                style={[styles.typeButton, feedbackType === type.type && styles.typeButtonActive]}
                onPress={() => setFeedbackType(type.type)}
              >
                <Text style={styles.typeIcon}>{type.icon}</Text>
                <Text
                  style={[styles.typeLabel, feedbackType === type.type && styles.typeLabelActive]}
                >
                  {type.label}
                </Text>
              </Pressable>
            ))}
          </View>
        </View>

        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Title *</Text>
          <TextInput
            style={styles.input}
            placeholder="Brief summary of your feedback"
            value={title}
            onChangeText={setTitle}
            placeholderTextColor="#9ca3af"
          />
        </View>

        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Description *</Text>
          <TextInput
            style={[styles.input, styles.textArea]}
            placeholder="Please describe in detail..."
            value={description}
            onChangeText={setDescription}
            multiline
            numberOfLines={6}
            textAlignVertical="top"
            placeholderTextColor="#9ca3af"
          />
        </View>

        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Email (optional)</Text>
          <TextInput
            style={styles.input}
            placeholder="your@email.com"
            value={email}
            onChangeText={setEmail}
            keyboardType="email-address"
            autoCapitalize="none"
            autoCorrect={false}
            placeholderTextColor="#9ca3af"
          />
          <Text style={styles.helperText}>We may contact you for follow-up questions</Text>
        </View>

        <View style={styles.section}>
          <Pressable
            style={styles.checkboxRow}
            onPress={() => setIncludeDeviceInfo(!includeDeviceInfo)}
          >
            <View style={[styles.checkbox, includeDeviceInfo && styles.checkboxChecked]}>
              {includeDeviceInfo && <Text style={styles.checkmark}>✓</Text>}
            </View>
            <Text style={styles.checkboxLabel}>Include device information</Text>
          </Pressable>

          {includeDeviceInfo && (
            <View style={styles.deviceInfoPreview}>
              <Text style={styles.deviceInfoTitle}>Device Information</Text>
              <Text style={styles.deviceInfoText}>
                Platform: {deviceInfo.platform} {deviceInfo.osVersion}
              </Text>
              <Text style={styles.deviceInfoText}>
                App: v{deviceInfo.appVersion} (Build {deviceInfo.buildNumber})
              </Text>
              <Pressable
                style={styles.viewDiagnostics}
                onPress={() => Alert.alert('Diagnostic Report', diagnosticReport)}
              >
                <Text style={styles.viewDiagnosticsText}>View full diagnostics</Text>
              </Pressable>
            </View>
          )}
        </View>

        <View style={styles.buttonContainer}>
          <Pressable
            style={[styles.submitButton, isSubmitting && styles.submitButtonDisabled]}
            onPress={handleSubmit}
            disabled={isSubmitting}
          >
            <Text style={styles.submitButtonText}>
              {isSubmitting ? 'Submitting...' : 'Submit Feedback'}
            </Text>
          </Pressable>

          <Pressable style={styles.draftButton} onPress={handleSaveDraft}>
            <Text style={styles.draftButtonText}>Save Draft</Text>
          </Pressable>
        </View>

        <View style={styles.footer}>
          <Text style={styles.footerText}>
            Your feedback helps us improve the app. Thank you for taking the time to share your
            thoughts with us.
          </Text>
        </View>
      </ScrollView>
    </KeyboardAvoidingView>
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
  content: {
    flex: 1,
  },
  section: {
    backgroundColor: '#fff',
    marginTop: 16,
    paddingHorizontal: 16,
    paddingVertical: 16,
  },
  sectionTitle: {
    fontSize: 16,
    fontWeight: '600',
    color: '#111827',
    marginBottom: 12,
  },
  typeGrid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    gap: 8,
  },
  typeButton: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingHorizontal: 14,
    paddingVertical: 10,
    backgroundColor: '#f3f4f6',
    borderRadius: 8,
    borderWidth: 2,
    borderColor: 'transparent',
  },
  typeButtonActive: {
    backgroundColor: '#eff6ff',
    borderColor: '#2563eb',
  },
  typeIcon: {
    fontSize: 16,
    marginRight: 6,
  },
  typeLabel: {
    fontSize: 14,
    color: '#6b7280',
  },
  typeLabelActive: {
    color: '#2563eb',
    fontWeight: '500',
  },
  input: {
    backgroundColor: '#f3f4f6',
    borderRadius: 8,
    paddingHorizontal: 16,
    paddingVertical: 12,
    fontSize: 16,
    color: '#111827',
  },
  textArea: {
    minHeight: 120,
    paddingTop: 12,
  },
  helperText: {
    fontSize: 13,
    color: '#9ca3af',
    marginTop: 8,
  },
  checkboxRow: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  checkbox: {
    width: 24,
    height: 24,
    borderWidth: 2,
    borderColor: '#d1d5db',
    borderRadius: 4,
    marginRight: 12,
    alignItems: 'center',
    justifyContent: 'center',
  },
  checkboxChecked: {
    backgroundColor: '#2563eb',
    borderColor: '#2563eb',
  },
  checkmark: {
    color: '#fff',
    fontSize: 14,
    fontWeight: 'bold',
  },
  checkboxLabel: {
    fontSize: 16,
    color: '#111827',
  },
  deviceInfoPreview: {
    marginTop: 16,
    backgroundColor: '#f3f4f6',
    borderRadius: 8,
    padding: 12,
  },
  deviceInfoTitle: {
    fontSize: 14,
    fontWeight: '600',
    color: '#4b5563',
    marginBottom: 8,
  },
  deviceInfoText: {
    fontSize: 13,
    color: '#6b7280',
    marginBottom: 4,
  },
  viewDiagnostics: {
    marginTop: 8,
  },
  viewDiagnosticsText: {
    fontSize: 14,
    color: '#2563eb',
  },
  buttonContainer: {
    padding: 16,
    gap: 12,
  },
  submitButton: {
    backgroundColor: '#2563eb',
    paddingVertical: 16,
    borderRadius: 8,
    alignItems: 'center',
  },
  submitButtonDisabled: {
    backgroundColor: '#93c5fd',
  },
  submitButtonText: {
    color: '#fff',
    fontSize: 16,
    fontWeight: '600',
  },
  draftButton: {
    paddingVertical: 14,
    borderRadius: 8,
    alignItems: 'center',
    borderWidth: 1,
    borderColor: '#d1d5db',
  },
  draftButtonText: {
    color: '#6b7280',
    fontSize: 16,
    fontWeight: '500',
  },
  footer: {
    padding: 16,
    paddingBottom: 40,
  },
  footerText: {
    fontSize: 14,
    color: '#9ca3af',
    textAlign: 'center',
    lineHeight: 20,
  },
});
