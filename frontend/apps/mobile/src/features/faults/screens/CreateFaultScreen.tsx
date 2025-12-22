/**
 * CreateFaultScreen - screen for reporting a new fault.
 * Epic 4: Fault Reporting & Resolution (UC-03.1)
 */

import { useState } from 'react';
import {
  ActivityIndicator,
  KeyboardAvoidingView,
  Platform,
  ScrollView,
  StyleSheet,
  Text,
  TextInput,
  TouchableOpacity,
  View,
} from 'react-native';
import type { FaultCategory } from '../components/FaultCard';

interface CreateFaultScreenProps {
  buildings: Array<{ id: string; name: string }>;
  isSubmitting?: boolean;
  onSubmit: (data: {
    buildingId: string;
    title: string;
    description: string;
    category: FaultCategory;
    locationDescription?: string;
  }) => void;
  onCancel: () => void;
}

const categories: { value: FaultCategory; label: string }[] = [
  { value: 'plumbing', label: 'Plumbing' },
  { value: 'electrical', label: 'Electrical' },
  { value: 'heating', label: 'Heating' },
  { value: 'structural', label: 'Structural' },
  { value: 'exterior', label: 'Exterior' },
  { value: 'elevator', label: 'Elevator' },
  { value: 'common_area', label: 'Common Area' },
  { value: 'security', label: 'Security' },
  { value: 'cleaning', label: 'Cleaning' },
  { value: 'other', label: 'Other' },
];

export function CreateFaultScreen({
  buildings,
  isSubmitting,
  onSubmit,
  onCancel,
}: CreateFaultScreenProps) {
  const [buildingId, setBuildingId] = useState('');
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [category, setCategory] = useState<FaultCategory>('other');
  const [locationDescription, setLocationDescription] = useState('');
  const [errors, setErrors] = useState<{ [key: string]: string }>({});

  const validate = () => {
    const newErrors: { [key: string]: string } = {};
    if (!buildingId) newErrors.buildingId = 'Please select a building';
    if (!title.trim()) newErrors.title = 'Title is required';
    if (!description.trim()) newErrors.description = 'Description is required';
    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = () => {
    if (validate()) {
      onSubmit({
        buildingId,
        title: title.trim(),
        description: description.trim(),
        category,
        locationDescription: locationDescription.trim() || undefined,
      });
    }
  };

  return (
    <KeyboardAvoidingView
      style={styles.container}
      behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
    >
      <ScrollView style={styles.scrollView} contentContainerStyle={styles.content}>
        <Text style={styles.label}>Building *</Text>
        <View style={styles.pickerContainer}>
          {buildings.map((b) => (
            <TouchableOpacity
              key={b.id}
              style={[styles.pickerOption, buildingId === b.id && styles.pickerOptionSelected]}
              onPress={() => setBuildingId(b.id)}
            >
              <Text
                style={[
                  styles.pickerOptionText,
                  buildingId === b.id && styles.pickerOptionTextSelected,
                ]}
              >
                {b.name}
              </Text>
            </TouchableOpacity>
          ))}
        </View>
        {errors.buildingId && <Text style={styles.error}>{errors.buildingId}</Text>}

        <Text style={styles.label}>Title *</Text>
        <TextInput
          style={[styles.input, errors.title ? styles.inputError : undefined]}
          value={title}
          onChangeText={setTitle}
          placeholder="Brief description of the issue"
          maxLength={255}
        />
        {errors.title && <Text style={styles.error}>{errors.title}</Text>}

        <Text style={styles.label}>Description *</Text>
        <TextInput
          style={[styles.textArea, errors.description ? styles.inputError : undefined]}
          value={description}
          onChangeText={setDescription}
          placeholder="Provide detailed information..."
          multiline
          numberOfLines={4}
          textAlignVertical="top"
        />
        {errors.description && <Text style={styles.error}>{errors.description}</Text>}

        <Text style={styles.label}>Location (optional)</Text>
        <TextInput
          style={styles.input}
          value={locationDescription}
          onChangeText={setLocationDescription}
          placeholder="e.g., Kitchen sink, Hallway 3rd floor"
        />

        <Text style={styles.label}>Category</Text>
        <ScrollView horizontal showsHorizontalScrollIndicator={false} style={styles.categoryScroll}>
          {categories.map((cat) => (
            <TouchableOpacity
              key={cat.value}
              style={[styles.categoryChip, category === cat.value && styles.categoryChipSelected]}
              onPress={() => setCategory(cat.value)}
            >
              <Text
                style={[
                  styles.categoryChipText,
                  category === cat.value && styles.categoryChipTextSelected,
                ]}
              >
                {cat.label}
              </Text>
            </TouchableOpacity>
          ))}
        </ScrollView>
      </ScrollView>

      <View style={styles.footer}>
        <TouchableOpacity style={styles.cancelButton} onPress={onCancel} disabled={isSubmitting}>
          <Text style={styles.cancelButtonText}>Cancel</Text>
        </TouchableOpacity>
        <TouchableOpacity
          style={[styles.submitButton, isSubmitting && styles.submitButtonDisabled]}
          onPress={handleSubmit}
          disabled={isSubmitting}
        >
          {isSubmitting ? (
            <ActivityIndicator size="small" color="#FFFFFF" />
          ) : (
            <Text style={styles.submitButtonText}>Submit</Text>
          )}
        </TouchableOpacity>
      </View>
    </KeyboardAvoidingView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#FFFFFF',
  },
  scrollView: {
    flex: 1,
  },
  content: {
    padding: 16,
    paddingBottom: 100,
  },
  label: {
    fontSize: 14,
    fontWeight: '500',
    color: '#374151',
    marginTop: 16,
    marginBottom: 8,
  },
  input: {
    borderWidth: 1,
    borderColor: '#D1D5DB',
    borderRadius: 8,
    padding: 12,
    fontSize: 16,
    color: '#111827',
    backgroundColor: '#FFFFFF',
  },
  textArea: {
    borderWidth: 1,
    borderColor: '#D1D5DB',
    borderRadius: 8,
    padding: 12,
    fontSize: 16,
    color: '#111827',
    backgroundColor: '#FFFFFF',
    minHeight: 100,
  },
  inputError: {
    borderColor: '#EF4444',
  },
  error: {
    fontSize: 12,
    color: '#EF4444',
    marginTop: 4,
  },
  pickerContainer: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    gap: 8,
  },
  pickerOption: {
    paddingHorizontal: 12,
    paddingVertical: 8,
    borderRadius: 8,
    borderWidth: 1,
    borderColor: '#D1D5DB',
    backgroundColor: '#FFFFFF',
  },
  pickerOptionSelected: {
    backgroundColor: '#2563EB',
    borderColor: '#2563EB',
  },
  pickerOptionText: {
    fontSize: 14,
    color: '#374151',
  },
  pickerOptionTextSelected: {
    color: '#FFFFFF',
  },
  categoryScroll: {
    marginTop: 4,
  },
  categoryChip: {
    paddingHorizontal: 16,
    paddingVertical: 8,
    borderRadius: 20,
    backgroundColor: '#F3F4F6',
    marginRight: 8,
  },
  categoryChipSelected: {
    backgroundColor: '#2563EB',
  },
  categoryChipText: {
    fontSize: 14,
    color: '#374151',
  },
  categoryChipTextSelected: {
    color: '#FFFFFF',
  },
  footer: {
    position: 'absolute',
    bottom: 0,
    left: 0,
    right: 0,
    flexDirection: 'row',
    padding: 16,
    backgroundColor: '#FFFFFF',
    borderTopWidth: 1,
    borderTopColor: '#E5E7EB',
    gap: 12,
  },
  cancelButton: {
    flex: 1,
    paddingVertical: 14,
    borderRadius: 8,
    borderWidth: 1,
    borderColor: '#D1D5DB',
    alignItems: 'center',
  },
  cancelButtonText: {
    fontSize: 16,
    fontWeight: '500',
    color: '#374151',
  },
  submitButton: {
    flex: 1,
    paddingVertical: 14,
    borderRadius: 8,
    backgroundColor: '#2563EB',
    alignItems: 'center',
  },
  submitButtonDisabled: {
    opacity: 0.5,
  },
  submitButtonText: {
    fontSize: 16,
    fontWeight: '500',
    color: '#FFFFFF',
  },
});
