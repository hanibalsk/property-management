/**
 * Emergency management types for Epic 62.
 */

export interface EmergencyContact {
  id: string;
  organization_id: string;
  building_id?: string;
  name: string;
  role: string;
  phone?: string;
  phone_secondary?: string;
  email?: string;
  address?: string;
  notes?: string;
  priority_order: number;
  contact_type: string;
  is_active: boolean;
  available_hours?: string;
  created_at: string;
  updated_at: string;
}

export interface CreateEmergencyContact {
  building_id?: string;
  name: string;
  role: string;
  phone?: string;
  phone_secondary?: string;
  email?: string;
  address?: string;
  notes?: string;
  priority_order?: number;
  contact_type: string;
  available_hours?: string;
}

export interface UpdateEmergencyContact {
  building_id?: string;
  name?: string;
  role?: string;
  phone?: string;
  phone_secondary?: string;
  email?: string;
  address?: string;
  notes?: string;
  priority_order?: number;
  contact_type?: string;
  is_active?: boolean;
  available_hours?: string;
}

export interface EmergencyContactQuery {
  organization_id: string;
  building_id?: string;
  contact_type?: string;
  is_active?: boolean;
  limit?: number;
  offset?: number;
}

// Contact type constants
export const CONTACT_TYPES = {
  FIRE_DEPARTMENT: 'fire_department',
  POLICE: 'police',
  AMBULANCE: 'ambulance',
  UTILITY_COMPANY: 'utility_company',
  BUILDING_MANAGER: 'building_manager',
  SECURITY: 'security',
  MAINTENANCE: 'maintenance',
  MEDICAL: 'medical',
  OTHER: 'other',
} as const;

export type ContactType = (typeof CONTACT_TYPES)[keyof typeof CONTACT_TYPES];

export const CONTACT_TYPE_LABELS: Record<ContactType, string> = {
  [CONTACT_TYPES.FIRE_DEPARTMENT]: 'Fire Department',
  [CONTACT_TYPES.POLICE]: 'Police',
  [CONTACT_TYPES.AMBULANCE]: 'Ambulance',
  [CONTACT_TYPES.UTILITY_COMPANY]: 'Utility Company',
  [CONTACT_TYPES.BUILDING_MANAGER]: 'Building Manager',
  [CONTACT_TYPES.SECURITY]: 'Security',
  [CONTACT_TYPES.MAINTENANCE]: 'Maintenance',
  [CONTACT_TYPES.MEDICAL]: 'Medical',
  [CONTACT_TYPES.OTHER]: 'Other',
};
