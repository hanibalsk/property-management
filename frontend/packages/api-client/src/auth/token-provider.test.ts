/**
 * Token Provider Unit Tests (Epic 80, Story 80.4)
 *
 * Tests for the authentication token provider including:
 * - Token provider setup and teardown
 * - Token retrieval
 * - Provider state checks
 */

import { afterEach, describe, expect, it } from 'vitest';
import { clearTokenProvider, getToken, hasTokenProvider, setTokenProvider } from './token-provider';

describe('Token Provider', () => {
  afterEach(() => {
    // Clean up after each test
    clearTokenProvider();
  });

  describe('setTokenProvider', () => {
    it('sets the token provider function', () => {
      const mockProvider = () => 'test-token';
      setTokenProvider(mockProvider);
      expect(hasTokenProvider()).toBe(true);
    });

    it('allows updating the token provider', () => {
      const provider1 = () => 'token-1';
      const provider2 = () => 'token-2';

      setTokenProvider(provider1);
      expect(getToken()).toBe('token-1');

      setTokenProvider(provider2);
      expect(getToken()).toBe('token-2');
    });
  });

  describe('clearTokenProvider', () => {
    it('clears the token provider', () => {
      setTokenProvider(() => 'test-token');
      expect(hasTokenProvider()).toBe(true);

      clearTokenProvider();
      expect(hasTokenProvider()).toBe(false);
    });

    it('can be called when no provider is set', () => {
      // Should not throw
      expect(() => clearTokenProvider()).not.toThrow();
      expect(hasTokenProvider()).toBe(false);
    });
  });

  describe('getToken', () => {
    it('returns null when no provider is set', () => {
      expect(getToken()).toBeNull();
    });

    it('returns the token from the provider', () => {
      setTokenProvider(() => 'my-access-token');
      expect(getToken()).toBe('my-access-token');
    });

    it('returns null when provider returns null', () => {
      setTokenProvider(() => null);
      expect(getToken()).toBeNull();
    });

    it('calls the provider function each time', () => {
      let callCount = 0;
      setTokenProvider(() => {
        callCount++;
        return `token-${callCount}`;
      });

      expect(getToken()).toBe('token-1');
      expect(getToken()).toBe('token-2');
      expect(getToken()).toBe('token-3');
      expect(callCount).toBe(3);
    });
  });

  describe('hasTokenProvider', () => {
    it('returns false when no provider is set', () => {
      expect(hasTokenProvider()).toBe(false);
    });

    it('returns true when a provider is set', () => {
      setTokenProvider(() => 'token');
      expect(hasTokenProvider()).toBe(true);
    });

    it('returns false after clearing the provider', () => {
      setTokenProvider(() => 'token');
      clearTokenProvider();
      expect(hasTokenProvider()).toBe(false);
    });
  });
});
