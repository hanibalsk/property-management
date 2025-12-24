import * as ImagePicker from 'expo-image-picker';
import * as Location from 'expo-location';
import { useCallback, useState } from 'react';
import {
  ActivityIndicator,
  Alert,
  Image,
  KeyboardAvoidingView,
  Platform,
  Pressable,
  ScrollView,
  StyleSheet,
  Text,
  TextInput,
  View,
} from 'react-native';
import type { FaultCategory, FaultPriority } from './FaultsListScreen';

interface ReportFaultScreenProps {
  onSuccess?: () => void;
  onCancel?: () => void;
}

const categories: Array<{ value: FaultCategory; label: string; icon: string }> = [
  { value: 'plumbing', label: 'Plumbing', icon: '🚿' },
  { value: 'electrical', label: 'Electrical', icon: '⚡' },
  { value: 'structural', label: 'Structural', icon: '🏗️' },
  { value: 'hvac', label: 'HVAC', icon: '❄️' },
  { value: 'elevator', label: 'Elevator', icon: '🛗' },
  { value: 'security', label: 'Security', icon: '🔒' },
  { value: 'other', label: 'Other', icon: '🔧' },
];

const priorities: Array<{ value: FaultPriority; label: string; color: string }> = [
  { value: 'low', label: 'Low', color: '#65a30d' },
  { value: 'medium', label: 'Medium', color: '#ca8a04' },
  { value: 'high', label: 'High', color: '#ea580c' },
  { value: 'urgent', label: 'Urgent', color: '#dc2626' },
];

export function ReportFaultScreen({ onSuccess, onCancel }: ReportFaultScreenProps) {
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [category, setCategory] = useState<FaultCategory | null>(null);
  const [priority, setPriority] = useState<FaultPriority>('medium');
  const [location, setLocation] = useState('');
  const [photos, setPhotos] = useState<string[]>([]);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isDetectingLocation, setIsDetectingLocation] = useState(false);
  const [errors, setErrors] = useState<Record<string, string>>({});

  const validateForm = (): boolean => {
    const newErrors: Record<string, string> = {};

    if (!title.trim()) {
      newErrors.title = 'Title is required';
    } else if (title.length < 5) {
      newErrors.title = 'Title must be at least 5 characters';
    }

    if (!description.trim()) {
      newErrors.description = 'Description is required';
    } else if (description.length < 20) {
      newErrors.description = 'Description must be at least 20 characters';
    }

    if (!category) {
      newErrors.category = 'Please select a category';
    }

    if (!location.trim()) {
      newErrors.location = 'Location is required';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const pickImage = async (useCamera: boolean) => {
    try {
      if (useCamera) {
        const { status } = await ImagePicker.requestCameraPermissionsAsync();
        if (status !== 'granted') {
          Alert.alert('Permission Denied', 'Camera permission is required to take photos');
          return;
        }
      } else {
        const { status } = await ImagePicker.requestMediaLibraryPermissionsAsync();
        if (status !== 'granted') {
          Alert.alert('Permission Denied', 'Photo library permission is required');
          return;
        }
      }

      const result = await (useCamera
        ? ImagePicker.launchCameraAsync({
            mediaTypes: 'images',
            quality: 0.8,
            allowsEditing: true,
          })
        : ImagePicker.launchImageLibraryAsync({
            mediaTypes: 'images',
            quality: 0.8,
            allowsMultipleSelection: true,
            selectionLimit: 5 - photos.length,
          }));

      if (!result.canceled) {
        const newPhotos = result.assets.map((asset: { uri: string }) => asset.uri);
        setPhotos((prev) => [...prev, ...newPhotos].slice(0, 5));
      }
    } catch (_error) {
      Alert.alert('Error', 'Failed to pick image');
    }
  };

  const removePhoto = (index: number) => {
    setPhotos((prev) => prev.filter((_, i) => i !== index));
  };

  const detectLocation = useCallback(async () => {
    setIsDetectingLocation(true);
    try {
      const { status } = await Location.requestForegroundPermissionsAsync();
      if (status !== 'granted') {
        Alert.alert('Permission Denied', 'Location permission is required');
        return;
      }

      const currentLocation = await Location.getCurrentPositionAsync({});
      const [address] = await Location.reverseGeocodeAsync({
        latitude: currentLocation.coords.latitude,
        longitude: currentLocation.coords.longitude,
      });

      if (address) {
        const locationText = [address.street, address.city].filter(Boolean).join(', ');
        setLocation(locationText || 'Location detected');
      }
    } catch (_error) {
      Alert.alert('Error', 'Failed to detect location');
    } finally {
      setIsDetectingLocation(false);
    }
  }, []);

  const handleSubmit = async () => {
    if (!validateForm()) {
      return;
    }

    setIsSubmitting(true);

    try {
      // Simulate API call
      await new Promise((resolve) => setTimeout(resolve, 1500));

      Alert.alert('Success', 'Fault report submitted successfully', [
        { text: 'OK', onPress: onSuccess },
      ]);
    } catch (_error) {
      Alert.alert('Error', 'Failed to submit fault report');
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <KeyboardAvoidingView
      style={styles.container}
      behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
    >
      {/* Header */}
      <View style={styles.header}>
        <Pressable style={styles.cancelButton} onPress={onCancel}>
          <Text style={styles.cancelText}>Cancel</Text>
        </Pressable>
        <Text style={styles.headerTitle}>Report Fault</Text>
        <View style={styles.placeholder} />
      </View>

      <ScrollView style={styles.scrollView} keyboardShouldPersistTaps="handled">
        {/* Title */}
        <View style={styles.formGroup}>
          <Text style={styles.label}>Title *</Text>
          <TextInput
            style={[styles.input, errors.title ? styles.inputError : undefined]}
            placeholder="Brief description of the issue"
            value={title}
            onChangeText={setTitle}
            maxLength={100}
          />
          {errors.title && <Text style={styles.errorText}>{errors.title}</Text>}
        </View>

        {/* Category */}
        <View style={styles.formGroup}>
          <Text style={styles.label}>Category *</Text>
          <View style={styles.categoryGrid}>
            {categories.map((cat) => (
              <Pressable
                key={cat.value}
                style={[
                  styles.categoryButton,
                  category === cat.value && styles.categoryButtonSelected,
                ]}
                onPress={() => setCategory(cat.value)}
              >
                <Text style={styles.categoryIcon}>{cat.icon}</Text>
                <Text
                  style={[
                    styles.categoryLabel,
                    category === cat.value && styles.categoryLabelSelected,
                  ]}
                >
                  {cat.label}
                </Text>
              </Pressable>
            ))}
          </View>
          {errors.category && <Text style={styles.errorText}>{errors.category}</Text>}
        </View>

        {/* Priority */}
        <View style={styles.formGroup}>
          <Text style={styles.label}>Priority</Text>
          <View style={styles.priorityRow}>
            {priorities.map((p) => (
              <Pressable
                key={p.value}
                style={[
                  styles.priorityButton,
                  priority === p.value && { backgroundColor: p.color, borderColor: p.color },
                ]}
                onPress={() => setPriority(p.value)}
              >
                <Text
                  style={[
                    styles.priorityLabel,
                    priority === p.value && styles.priorityLabelSelected,
                  ]}
                >
                  {p.label}
                </Text>
              </Pressable>
            ))}
          </View>
        </View>

        {/* Location */}
        <View style={styles.formGroup}>
          <Text style={styles.label}>Location *</Text>
          <View style={styles.locationRow}>
            <TextInput
              style={[
                styles.input,
                styles.locationInput,
                errors.location ? styles.inputError : undefined,
              ]}
              placeholder="e.g., 3rd floor, Unit 301"
              value={location}
              onChangeText={setLocation}
            />
            <Pressable
              style={styles.detectButton}
              onPress={detectLocation}
              disabled={isDetectingLocation}
            >
              {isDetectingLocation ? (
                <ActivityIndicator size="small" color="#2563eb" />
              ) : (
                <Text style={styles.detectButtonText}>📍</Text>
              )}
            </Pressable>
          </View>
          {errors.location && <Text style={styles.errorText}>{errors.location}</Text>}
        </View>

        {/* Description */}
        <View style={styles.formGroup}>
          <Text style={styles.label}>Description *</Text>
          <TextInput
            style={[
              styles.input,
              styles.textArea,
              errors.description ? styles.inputError : undefined,
            ]}
            placeholder="Provide detailed information about the issue..."
            value={description}
            onChangeText={setDescription}
            multiline
            numberOfLines={4}
            textAlignVertical="top"
          />
          {errors.description && <Text style={styles.errorText}>{errors.description}</Text>}
        </View>

        {/* Photos */}
        <View style={styles.formGroup}>
          <Text style={styles.label}>Photos (optional)</Text>
          <View style={styles.photosContainer}>
            {photos.map((photo, index) => (
              <View key={photo} style={styles.photoWrapper}>
                <Image source={{ uri: photo }} style={styles.photoPreview} />
                <Pressable style={styles.removePhoto} onPress={() => removePhoto(index)}>
                  <Text style={styles.removePhotoText}>✕</Text>
                </Pressable>
              </View>
            ))}
            {photos.length < 5 && (
              <View style={styles.photoActions}>
                <Pressable style={styles.photoButton} onPress={() => pickImage(true)}>
                  <Text style={styles.photoButtonIcon}>📷</Text>
                  <Text style={styles.photoButtonText}>Camera</Text>
                </Pressable>
                <Pressable style={styles.photoButton} onPress={() => pickImage(false)}>
                  <Text style={styles.photoButtonIcon}>🖼️</Text>
                  <Text style={styles.photoButtonText}>Gallery</Text>
                </Pressable>
              </View>
            )}
          </View>
          <Text style={styles.photoHint}>{photos.length}/5 photos</Text>
        </View>

        {/* Submit Button */}
        <Pressable
          style={[styles.submitButton, isSubmitting && styles.submitButtonDisabled]}
          onPress={handleSubmit}
          disabled={isSubmitting}
        >
          {isSubmitting ? (
            <ActivityIndicator color="#fff" />
          ) : (
            <Text style={styles.submitButtonText}>Submit Report</Text>
          )}
        </Pressable>

        <View style={styles.bottomSpacer} />
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
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    padding: 20,
    paddingTop: 60,
    backgroundColor: '#fff',
    borderBottomWidth: 1,
    borderBottomColor: '#e5e7eb',
  },
  cancelButton: {
    padding: 4,
  },
  cancelText: {
    color: '#6b7280',
    fontSize: 16,
  },
  headerTitle: {
    fontSize: 18,
    fontWeight: '600',
    color: '#1f2937',
  },
  placeholder: {
    width: 50,
  },
  scrollView: {
    flex: 1,
    padding: 16,
  },
  formGroup: {
    marginBottom: 20,
  },
  label: {
    fontSize: 14,
    fontWeight: '600',
    color: '#374151',
    marginBottom: 8,
  },
  input: {
    backgroundColor: '#fff',
    borderRadius: 8,
    borderWidth: 1,
    borderColor: '#d1d5db',
    padding: 12,
    fontSize: 16,
  },
  inputError: {
    borderColor: '#ef4444',
  },
  errorText: {
    color: '#ef4444',
    fontSize: 12,
    marginTop: 4,
  },
  textArea: {
    minHeight: 100,
  },
  categoryGrid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    gap: 8,
  },
  categoryButton: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingHorizontal: 12,
    paddingVertical: 8,
    borderRadius: 8,
    backgroundColor: '#fff',
    borderWidth: 1,
    borderColor: '#d1d5db',
    gap: 6,
  },
  categoryButtonSelected: {
    backgroundColor: '#eff6ff',
    borderColor: '#2563eb',
  },
  categoryIcon: {
    fontSize: 18,
  },
  categoryLabel: {
    fontSize: 14,
    color: '#374151',
  },
  categoryLabelSelected: {
    color: '#2563eb',
    fontWeight: '500',
  },
  priorityRow: {
    flexDirection: 'row',
    gap: 8,
  },
  priorityButton: {
    flex: 1,
    paddingVertical: 10,
    borderRadius: 8,
    backgroundColor: '#fff',
    borderWidth: 1,
    borderColor: '#d1d5db',
    alignItems: 'center',
  },
  priorityLabel: {
    fontSize: 14,
    color: '#374151',
    fontWeight: '500',
  },
  priorityLabelSelected: {
    color: '#fff',
  },
  locationRow: {
    flexDirection: 'row',
    gap: 8,
  },
  locationInput: {
    flex: 1,
  },
  detectButton: {
    backgroundColor: '#fff',
    borderRadius: 8,
    borderWidth: 1,
    borderColor: '#d1d5db',
    width: 48,
    alignItems: 'center',
    justifyContent: 'center',
  },
  detectButtonText: {
    fontSize: 20,
  },
  photosContainer: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    gap: 8,
  },
  photoWrapper: {
    position: 'relative',
  },
  photoPreview: {
    width: 80,
    height: 80,
    borderRadius: 8,
  },
  removePhoto: {
    position: 'absolute',
    top: -6,
    right: -6,
    backgroundColor: '#ef4444',
    width: 22,
    height: 22,
    borderRadius: 11,
    alignItems: 'center',
    justifyContent: 'center',
  },
  removePhotoText: {
    color: '#fff',
    fontSize: 12,
    fontWeight: 'bold',
  },
  photoActions: {
    flexDirection: 'row',
    gap: 8,
  },
  photoButton: {
    width: 80,
    height: 80,
    backgroundColor: '#fff',
    borderRadius: 8,
    borderWidth: 1,
    borderColor: '#d1d5db',
    borderStyle: 'dashed',
    alignItems: 'center',
    justifyContent: 'center',
  },
  photoButtonIcon: {
    fontSize: 24,
    marginBottom: 4,
  },
  photoButtonText: {
    fontSize: 12,
    color: '#6b7280',
  },
  photoHint: {
    fontSize: 12,
    color: '#9ca3af',
    marginTop: 6,
  },
  submitButton: {
    backgroundColor: '#2563eb',
    borderRadius: 8,
    padding: 16,
    alignItems: 'center',
    marginTop: 8,
  },
  submitButtonDisabled: {
    backgroundColor: '#93c5fd',
  },
  submitButtonText: {
    color: '#fff',
    fontSize: 16,
    fontWeight: '600',
  },
  bottomSpacer: {
    height: 40,
  },
});
