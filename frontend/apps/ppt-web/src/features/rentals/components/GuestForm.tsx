/**
 * GuestForm Component
 *
 * Form to add/edit guest information for government reporting.
 * Epic 18: Short-Term Rental Integration (Story 18.3)
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { CreateGuestRequest, RentalGuest, UpdateGuestRequest } from '../types';

interface GuestFormProps {
  bookingId: string;
  initialData?: RentalGuest;
  isEditing?: boolean;
  isSubmitting?: boolean;
  onSubmit: (data: CreateGuestRequest | UpdateGuestRequest) => void;
  onCancel: () => void;
}

const documentTypes = ['passport', 'id_card', 'drivers_license', 'residence_permit', 'other'];

const nationalities = [
  'AT',
  'BE',
  'BG',
  'CY',
  'CZ',
  'DE',
  'DK',
  'EE',
  'ES',
  'FI',
  'FR',
  'GR',
  'HR',
  'HU',
  'IE',
  'IT',
  'LT',
  'LU',
  'LV',
  'MT',
  'NL',
  'PL',
  'PT',
  'RO',
  'SE',
  'SI',
  'SK',
  'GB',
  'US',
  'CA',
  'AU',
  'JP',
  'KR',
  'CN',
  'RU',
  'UA',
  'OTHER',
];

export function GuestForm({
  bookingId,
  initialData,
  isEditing = false,
  isSubmitting = false,
  onSubmit,
  onCancel,
}: GuestFormProps) {
  const { t } = useTranslation();

  const [firstName, setFirstName] = useState(initialData?.firstName || '');
  const [lastName, setLastName] = useState(initialData?.lastName || '');
  const [email, setEmail] = useState(initialData?.email || '');
  const [phone, setPhone] = useState(initialData?.phone || '');
  const [dateOfBirth, setDateOfBirth] = useState(initialData?.dateOfBirth?.slice(0, 10) || '');
  const [nationality, setNationality] = useState(initialData?.nationality || '');
  const [documentType, setDocumentType] = useState(initialData?.documentType || '');
  const [documentNumber, setDocumentNumber] = useState(initialData?.documentNumber || '');
  const [documentExpiry, setDocumentExpiry] = useState(
    initialData?.documentExpiry?.slice(0, 10) || ''
  );
  const [address, setAddress] = useState(initialData?.address || '');
  const [city, setCity] = useState(initialData?.city || '');
  const [country, setCountry] = useState(initialData?.country || '');
  const [isPrimary, setIsPrimary] = useState(initialData?.isPrimary ?? true);
  const [errors, setErrors] = useState<Record<string, string>>({});

  const validate = (): boolean => {
    const newErrors: Record<string, string> = {};

    if (!firstName.trim()) {
      newErrors.firstName = t('rentals.guest.errors.firstNameRequired');
    }

    if (!lastName.trim()) {
      newErrors.lastName = t('rentals.guest.errors.lastNameRequired');
    }

    if (!dateOfBirth) {
      newErrors.dateOfBirth = t('rentals.guest.errors.dateOfBirthRequired');
    }

    if (!nationality) {
      newErrors.nationality = t('rentals.guest.errors.nationalityRequired');
    }

    if (!documentType) {
      newErrors.documentType = t('rentals.guest.errors.documentTypeRequired');
    }

    if (!documentNumber.trim()) {
      newErrors.documentNumber = t('rentals.guest.errors.documentNumberRequired');
    }

    if (email && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
      newErrors.email = t('rentals.guest.errors.invalidEmail');
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (validate()) {
      if (isEditing) {
        const data: UpdateGuestRequest = {
          firstName: firstName.trim(),
          lastName: lastName.trim(),
          email: email.trim() || undefined,
          phone: phone.trim() || undefined,
          dateOfBirth: dateOfBirth || undefined,
          nationality: nationality || undefined,
          documentType: documentType || undefined,
          documentNumber: documentNumber.trim() || undefined,
          documentExpiry: documentExpiry || undefined,
          address: address.trim() || undefined,
          city: city.trim() || undefined,
          country: country || undefined,
        };
        onSubmit(data);
      } else {
        const data: CreateGuestRequest = {
          bookingId,
          firstName: firstName.trim(),
          lastName: lastName.trim(),
          email: email.trim() || undefined,
          phone: phone.trim() || undefined,
          dateOfBirth: dateOfBirth || undefined,
          nationality: nationality || undefined,
          documentType: documentType || undefined,
          documentNumber: documentNumber.trim() || undefined,
          documentExpiry: documentExpiry || undefined,
          address: address.trim() || undefined,
          city: city.trim() || undefined,
          country: country || undefined,
          isPrimary,
        };
        onSubmit(data);
      }
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Personal Information */}
      <div className="bg-gray-50 rounded-lg p-4">
        <h3 className="text-sm font-medium text-gray-700 mb-4">
          {t('rentals.guest.personalInfo')}
        </h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label htmlFor="firstName" className="block text-sm font-medium text-gray-700">
              {t('rentals.guest.firstName')} <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              id="firstName"
              value={firstName}
              onChange={(e) => setFirstName(e.target.value)}
              className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
                errors.firstName ? 'border-red-300' : 'border-gray-300'
              }`}
            />
            {errors.firstName && <p className="mt-1 text-sm text-red-600">{errors.firstName}</p>}
          </div>
          <div>
            <label htmlFor="lastName" className="block text-sm font-medium text-gray-700">
              {t('rentals.guest.lastName')} <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              id="lastName"
              value={lastName}
              onChange={(e) => setLastName(e.target.value)}
              className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
                errors.lastName ? 'border-red-300' : 'border-gray-300'
              }`}
            />
            {errors.lastName && <p className="mt-1 text-sm text-red-600">{errors.lastName}</p>}
          </div>
          <div>
            <label htmlFor="dateOfBirth" className="block text-sm font-medium text-gray-700">
              {t('rentals.guest.dateOfBirth')} <span className="text-red-500">*</span>
            </label>
            <input
              type="date"
              id="dateOfBirth"
              value={dateOfBirth}
              onChange={(e) => setDateOfBirth(e.target.value)}
              className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
                errors.dateOfBirth ? 'border-red-300' : 'border-gray-300'
              }`}
            />
            {errors.dateOfBirth && (
              <p className="mt-1 text-sm text-red-600">{errors.dateOfBirth}</p>
            )}
          </div>
          <div>
            <label htmlFor="nationality" className="block text-sm font-medium text-gray-700">
              {t('rentals.guest.nationality')} <span className="text-red-500">*</span>
            </label>
            <select
              id="nationality"
              value={nationality}
              onChange={(e) => setNationality(e.target.value)}
              className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
                errors.nationality ? 'border-red-300' : 'border-gray-300'
              }`}
            >
              <option value="">{t('common.select')}</option>
              {nationalities.map((code) => (
                <option key={code} value={code}>
                  {t(`rentals.guest.countries.${code}`)}
                </option>
              ))}
            </select>
            {errors.nationality && (
              <p className="mt-1 text-sm text-red-600">{errors.nationality}</p>
            )}
          </div>
        </div>
      </div>

      {/* Contact Information */}
      <div className="bg-gray-50 rounded-lg p-4">
        <h3 className="text-sm font-medium text-gray-700 mb-4">{t('rentals.guest.contactInfo')}</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label htmlFor="email" className="block text-sm font-medium text-gray-700">
              {t('rentals.guest.email')}
            </label>
            <input
              type="email"
              id="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
                errors.email ? 'border-red-300' : 'border-gray-300'
              }`}
            />
            {errors.email && <p className="mt-1 text-sm text-red-600">{errors.email}</p>}
          </div>
          <div>
            <label htmlFor="phone" className="block text-sm font-medium text-gray-700">
              {t('rentals.guest.phone')}
            </label>
            <input
              type="tel"
              id="phone"
              value={phone}
              onChange={(e) => setPhone(e.target.value)}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
        </div>
      </div>

      {/* Document Information */}
      <div className="bg-gray-50 rounded-lg p-4">
        <h3 className="text-sm font-medium text-gray-700 mb-4">
          {t('rentals.guest.documentInfo')}
        </h3>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div>
            <label htmlFor="documentType" className="block text-sm font-medium text-gray-700">
              {t('rentals.guest.documentType')} <span className="text-red-500">*</span>
            </label>
            <select
              id="documentType"
              value={documentType}
              onChange={(e) => setDocumentType(e.target.value)}
              className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
                errors.documentType ? 'border-red-300' : 'border-gray-300'
              }`}
            >
              <option value="">{t('common.select')}</option>
              {documentTypes.map((type) => (
                <option key={type} value={type}>
                  {t(`rentals.guest.documentTypes.${type}`)}
                </option>
              ))}
            </select>
            {errors.documentType && (
              <p className="mt-1 text-sm text-red-600">{errors.documentType}</p>
            )}
          </div>
          <div>
            <label htmlFor="documentNumber" className="block text-sm font-medium text-gray-700">
              {t('rentals.guest.documentNumber')} <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              id="documentNumber"
              value={documentNumber}
              onChange={(e) => setDocumentNumber(e.target.value)}
              className={`mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 ${
                errors.documentNumber ? 'border-red-300' : 'border-gray-300'
              }`}
            />
            {errors.documentNumber && (
              <p className="mt-1 text-sm text-red-600">{errors.documentNumber}</p>
            )}
          </div>
          <div>
            <label htmlFor="documentExpiry" className="block text-sm font-medium text-gray-700">
              {t('rentals.guest.documentExpiry')}
            </label>
            <input
              type="date"
              id="documentExpiry"
              value={documentExpiry}
              onChange={(e) => setDocumentExpiry(e.target.value)}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
        </div>
      </div>

      {/* Address Information */}
      <div className="bg-gray-50 rounded-lg p-4">
        <h3 className="text-sm font-medium text-gray-700 mb-4">{t('rentals.guest.addressInfo')}</h3>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="md:col-span-2">
            <label htmlFor="address" className="block text-sm font-medium text-gray-700">
              {t('rentals.guest.address')}
            </label>
            <input
              type="text"
              id="address"
              value={address}
              onChange={(e) => setAddress(e.target.value)}
              placeholder={t('rentals.guest.addressPlaceholder')}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
          <div>
            <label htmlFor="city" className="block text-sm font-medium text-gray-700">
              {t('rentals.guest.city')}
            </label>
            <input
              type="text"
              id="city"
              value={city}
              onChange={(e) => setCity(e.target.value)}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
          <div>
            <label htmlFor="country" className="block text-sm font-medium text-gray-700">
              {t('rentals.guest.country')}
            </label>
            <select
              id="country"
              value={country}
              onChange={(e) => setCountry(e.target.value)}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="">{t('common.select')}</option>
              {nationalities.map((code) => (
                <option key={code} value={code}>
                  {t(`rentals.guest.countries.${code}`)}
                </option>
              ))}
            </select>
          </div>
        </div>
      </div>

      {/* Primary Guest Toggle */}
      {!isEditing && (
        <div className="flex items-center">
          <input
            type="checkbox"
            id="isPrimary"
            checked={isPrimary}
            onChange={(e) => setIsPrimary(e.target.checked)}
            className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
          />
          <label htmlFor="isPrimary" className="ml-2 text-sm text-gray-700">
            {t('rentals.guest.isPrimary')}
          </label>
        </div>
      )}

      {/* Actions */}
      <div className="flex items-center justify-end gap-3 pt-4 border-t">
        <button
          type="button"
          onClick={onCancel}
          disabled={isSubmitting}
          className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50"
        >
          {t('common.cancel')}
        </button>
        <button
          type="submit"
          disabled={isSubmitting}
          className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50"
        >
          {isSubmitting
            ? isEditing
              ? t('rentals.guest.updating')
              : t('rentals.guest.adding')
            : isEditing
              ? t('rentals.guest.updateGuest')
              : t('rentals.guest.addGuest')}
        </button>
      </div>
    </form>
  );
}
