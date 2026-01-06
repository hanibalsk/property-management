/**
 * FAQPage - frequently asked questions page.
 * UC-42: Onboarding/Help Feature
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { FAQCategory, FAQItem } from '../types';

export interface FAQPageProps {
  faqs: FAQItem[];
  isLoading?: boolean;
  onNavigateBack: () => void;
  onNavigateToSupport: () => void;
}

export function FAQPage({ faqs, isLoading, onNavigateBack, onNavigateToSupport }: FAQPageProps) {
  const { t } = useTranslation();
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedCategory, setSelectedCategory] = useState<FAQCategory>('all');
  const [expandedIds, setExpandedIds] = useState<Set<string>>(new Set());

  const categories: { value: FAQCategory; label: string }[] = [
    { value: 'all', label: t('help.faq.categoryAll') },
    { value: 'account', label: t('help.faq.categoryAccount') },
    { value: 'billing', label: t('help.faq.categoryBilling') },
    { value: 'features', label: t('help.faq.categoryFeatures') },
    { value: 'technical', label: t('help.faq.categoryTechnical') },
    { value: 'general', label: t('help.faq.categoryGeneral') },
  ];

  const filteredFaqs = faqs.filter((faq) => {
    const matchesCategory = selectedCategory === 'all' || faq.category === selectedCategory;
    const matchesSearch =
      !searchQuery ||
      faq.question.toLowerCase().includes(searchQuery.toLowerCase()) ||
      faq.answer.toLowerCase().includes(searchQuery.toLowerCase());
    return matchesCategory && matchesSearch;
  });

  const toggleExpanded = (id: string) => {
    setExpandedIds((prev) => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  };

  const expandAll = () => {
    setExpandedIds(new Set(filteredFaqs.map((faq) => faq.id)));
  };

  const collapseAll = () => {
    setExpandedIds(new Set());
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" />
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto px-4 py-8">
      {/* Header */}
      <div className="mb-6">
        <button
          type="button"
          onClick={onNavigateBack}
          className="flex items-center text-sm text-gray-600 hover:text-gray-800 mb-4"
        >
          <svg className="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
            <title>{t('common.back')}</title>
            <path
              fillRule="evenodd"
              d="M9.707 16.707a1 1 0 01-1.414 0l-6-6a1 1 0 010-1.414l6-6a1 1 0 011.414 1.414L5.414 9H17a1 1 0 110 2H5.414l4.293 4.293a1 1 0 010 1.414z"
              clipRule="evenodd"
            />
          </svg>
          {t('help.backToHelpCenter')}
        </button>

        <h1 className="text-2xl font-bold text-gray-900">{t('help.faq.title')}</h1>
        <p className="mt-1 text-sm text-gray-500">{t('help.faq.subtitle')}</p>
      </div>

      {/* Search and filter */}
      <div className="mb-6 space-y-4">
        <div className="relative">
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder={t('help.faq.searchPlaceholder')}
            className="w-full px-4 py-2 pl-10 text-gray-900 bg-white border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          />
          <svg
            className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <title>{t('common.search')}</title>
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
        </div>

        <div className="flex flex-wrap items-center gap-2">
          <span className="text-sm text-gray-600">{t('help.faq.filterByCategory')}:</span>
          {categories.map((category) => (
            <button
              key={category.value}
              type="button"
              onClick={() => setSelectedCategory(category.value)}
              className={`px-3 py-1 text-sm rounded-full transition-colors ${
                selectedCategory === category.value
                  ? 'bg-blue-600 text-white'
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
              }`}
            >
              {category.label}
            </button>
          ))}
        </div>
      </div>

      {/* Expand/Collapse controls */}
      <div className="flex justify-end gap-2 mb-4">
        <button
          type="button"
          onClick={expandAll}
          className="text-sm text-blue-600 hover:text-blue-800"
        >
          {t('help.faq.expandAll')}
        </button>
        <span className="text-gray-300">|</span>
        <button
          type="button"
          onClick={collapseAll}
          className="text-sm text-blue-600 hover:text-blue-800"
        >
          {t('help.faq.collapseAll')}
        </button>
      </div>

      {/* FAQ accordion */}
      {filteredFaqs.length === 0 ? (
        <div className="text-center py-12 bg-white rounded-lg shadow">
          <svg
            className="mx-auto h-12 w-12 text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <title>{t('help.faq.noResults')}</title>
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={1.5}
              d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          <p className="mt-4 text-gray-500">{t('help.faq.noResults')}</p>
          <p className="mt-2 text-sm text-gray-400">{t('help.faq.noResultsDescription')}</p>
        </div>
      ) : (
        <div className="space-y-3">
          {filteredFaqs.map((faq) => (
            <FAQAccordionItem
              key={faq.id}
              faq={faq}
              isExpanded={expandedIds.has(faq.id)}
              onToggle={() => toggleExpanded(faq.id)}
            />
          ))}
        </div>
      )}

      {/* Contact support section */}
      <div className="mt-8 bg-gray-50 rounded-lg p-6 text-center">
        <svg
          className="mx-auto h-10 w-10 text-gray-400 mb-3"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <title>{t('help.faq.cantFindAnswer')}</title>
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={1.5}
            d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
        <h3 className="text-lg font-medium text-gray-900 mb-2">{t('help.faq.cantFindAnswer')}</h3>
        <p className="text-gray-600 mb-4">{t('help.faq.cantFindAnswerDescription')}</p>
        <button
          type="button"
          onClick={onNavigateToSupport}
          className="px-6 py-2 bg-blue-600 text-white font-medium rounded-md hover:bg-blue-700"
        >
          {t('help.contactSupport')}
        </button>
      </div>
    </div>
  );
}

interface FAQAccordionItemProps {
  faq: FAQItem;
  isExpanded: boolean;
  onToggle: () => void;
}

function FAQAccordionItem({ faq, isExpanded, onToggle }: FAQAccordionItemProps) {
  const { t } = useTranslation();

  const categoryLabels: Record<FAQCategory, string> = {
    all: t('help.faq.categoryAll'),
    account: t('help.faq.categoryAccount'),
    billing: t('help.faq.categoryBilling'),
    features: t('help.faq.categoryFeatures'),
    technical: t('help.faq.categoryTechnical'),
    general: t('help.faq.categoryGeneral'),
  };

  return (
    <div className="bg-white rounded-lg border border-gray-200 shadow-sm overflow-hidden">
      <button
        type="button"
        onClick={onToggle}
        className="w-full px-6 py-4 flex items-center justify-between text-left hover:bg-gray-50"
      >
        <div className="flex-1 pr-4">
          <div className="flex items-center gap-2 mb-1">
            <span className="text-xs px-2 py-0.5 bg-gray-100 text-gray-600 rounded">
              {categoryLabels[faq.category]}
            </span>
            {faq.isFeatured && (
              <span className="text-xs px-2 py-0.5 bg-blue-100 text-blue-600 rounded">
                {t('help.faq.featured')}
              </span>
            )}
          </div>
          <h3 className="font-medium text-gray-900">{faq.question}</h3>
        </div>
        <svg
          className={`w-5 h-5 text-gray-500 transition-transform ${isExpanded ? 'rotate-180' : ''}`}
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <title>{isExpanded ? t('help.faq.collapse') : t('help.faq.expand')}</title>
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
      </button>
      {isExpanded && (
        <div className="px-6 pb-4 border-t border-gray-100">
          <div className="pt-4 text-gray-600 prose prose-sm max-w-none">{faq.answer}</div>
        </div>
      )}
    </div>
  );
}
