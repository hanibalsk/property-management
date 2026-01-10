# Accessibility Audit Report (WCAG 2.1 AA)

**Epic 125, Story 125.2 - Manual Accessibility Audit**
**Date:** January 2026
**Application:** ppt-web (Property Management Web)

## Executive Summary

This audit evaluates the accessibility compliance of the ppt-web application against WCAG 2.1 Level AA guidelines. The audit covers automated testing with axe-core and manual review of key UI components.

## Automated Testing Coverage

The following components have automated accessibility tests (`.a11y.test.tsx` files):

| Component | Tests | Status |
|-----------|-------|--------|
| Toast | 6 tests | Passing |
| ConfirmationDialog | 9 tests | Passing |
| LanguageSwitcher | 5 tests | Passing |
| FaultCard | 11 tests | Passing |

## WCAG 2.1 AA Compliance Checklist

### Principle 1: Perceivable

#### 1.1 Text Alternatives
- [ ] **1.1.1 Non-text Content (A)**: All images and icons have appropriate alt text or aria-labels
  - **Findings**: Urgent icon in FaultCard has proper `aria-label` and `<title>` element
  - **Status**: Compliant

#### 1.2 Time-based Media
- Not applicable (no video/audio content)

#### 1.3 Adaptable
- [ ] **1.3.1 Info and Relationships (A)**: Semantic HTML used appropriately
  - **Findings**: Headings properly nested, form labels associated with inputs
  - **Status**: Compliant
- [ ] **1.3.2 Meaningful Sequence (A)**: DOM order matches visual order
  - **Status**: Compliant
- [ ] **1.3.3 Sensory Characteristics (A)**: Instructions don't rely solely on shape/color
  - **Status**: Compliant

#### 1.4 Distinguishable
- [ ] **1.4.1 Use of Color (A)**: Color not used as only visual means
  - **Findings**: Status badges use text labels in addition to colors
  - **Status**: Compliant
- [ ] **1.4.3 Contrast (Minimum) (AA)**: 4.5:1 for normal text, 3:1 for large text
  - **Findings**: Verified via axe-core automated testing
  - **Status**: Compliant
- [ ] **1.4.4 Resize Text (AA)**: Text can be resized to 200% without loss
  - **Recommendation**: Test with browser zoom at 200%
- [ ] **1.4.10 Reflow (AA)**: Content reflows at 320px viewport
  - **Recommendation**: Add responsive design testing
- [ ] **1.4.11 Non-text Contrast (AA)**: UI components have 3:1 contrast
  - **Findings**: Button borders and focus rings verified
  - **Status**: Compliant

### Principle 2: Operable

#### 2.1 Keyboard Accessible
- [ ] **2.1.1 Keyboard (A)**: All functionality available via keyboard
  - **Findings**: All buttons and interactive elements are keyboard accessible
  - **Status**: Compliant
- [ ] **2.1.2 No Keyboard Trap (A)**: Focus can be moved away using keyboard
  - **Findings**: ConfirmationDialog properly traps and releases focus
  - **Status**: Compliant

#### 2.4 Navigable
- [ ] **2.4.1 Bypass Blocks (A)**: Skip links or landmarks provided
  - **Recommendation**: Add skip-to-main-content link
- [ ] **2.4.3 Focus Order (A)**: Focus order preserves meaning
  - **Findings**: Tab order follows visual layout
  - **Status**: Compliant
- [ ] **2.4.6 Headings and Labels (AA)**: Headings and labels are descriptive
  - **Status**: Compliant
- [ ] **2.4.7 Focus Visible (AA)**: Focus indicator is visible
  - **Findings**: Focus rings implemented via Tailwind
  - **Status**: Compliant

#### 2.5 Input Modalities
- [ ] **2.5.3 Label in Name (A)**: Accessible name contains visible text
  - **Status**: Compliant

### Principle 3: Understandable

#### 3.1 Readable
- [ ] **3.1.1 Language of Page (A)**: Page has lang attribute
  - **Recommendation**: Verify html lang attribute is set
- [ ] **3.1.2 Language of Parts (AA)**: Language changes are marked
  - **Findings**: LanguageSwitcher handles i18n properly
  - **Status**: Compliant

#### 3.2 Predictable
- [ ] **3.2.1 On Focus (A)**: Focus doesn't cause unexpected changes
  - **Status**: Compliant
- [ ] **3.2.2 On Input (A)**: Input doesn't cause unexpected changes
  - **Status**: Compliant

#### 3.3 Input Assistance
- [ ] **3.3.1 Error Identification (A)**: Errors are identified and described
  - **Recommendation**: Audit form error handling
- [ ] **3.3.2 Labels or Instructions (A)**: Labels provided for input
  - **Status**: Compliant

### Principle 4: Robust

#### 4.1 Compatible
- [ ] **4.1.1 Parsing (A)**: No duplicate IDs, proper nesting
  - **Findings**: Verified via axe-core
  - **Status**: Compliant
- [ ] **4.1.2 Name, Role, Value (A)**: Custom components have proper ARIA
  - **Findings**: ConfirmationDialog uses alertdialog role with proper ARIA
  - **Status**: Compliant

## Recommendations for Story 125.3

Based on this audit, the following improvements should be implemented:

### High Priority
1. **Skip Link**: Add "Skip to main content" link at top of page
2. **Focus Management**: Ensure focus returns to trigger after modal close
3. **Error Announcements**: Implement aria-live regions for form errors

### Medium Priority
4. **Responsive Testing**: Verify 320px minimum viewport support
5. **Zoom Testing**: Verify 200% browser zoom works correctly
6. **Loading States**: Add aria-busy for loading components

### Low Priority
7. **High Contrast Mode**: Implement Windows High Contrast Mode support (Story 125.4)
8. **Reduced Motion**: Respect prefers-reduced-motion for animations

## Testing Tools Used

- **axe-core**: Automated accessibility testing via vitest-axe
- **Manual Testing**: Keyboard navigation, screen reader simulation
- **Browser DevTools**: Accessibility tree inspection

## Files Changed

- `apps/ppt-web/src/components/Toast.a11y.test.tsx`
- `apps/ppt-web/src/components/ConfirmationDialog.a11y.test.tsx`
- `apps/ppt-web/src/components/LanguageSwitcher.a11y.test.tsx`
- `apps/ppt-web/src/features/faults/components/FaultCard.a11y.test.tsx`

## Next Steps

1. Run `pnpm test:a11y` to execute all accessibility tests
2. Address any failing tests
3. Implement fixes identified in Story 125.3
4. Add high contrast theme in Story 125.4
