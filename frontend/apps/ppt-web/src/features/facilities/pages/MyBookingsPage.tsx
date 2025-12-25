/**
 * My Bookings Page (Epic 56: Facility Booking).
 *
 * Shows the current user's facility bookings.
 */

import type { BookingStatus, BookingWithDetails } from '@ppt/api-client';
import { cancelBooking, getMyBookings } from '@ppt/api-client';
import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { BookingList, CancelBookingDialog } from '../components';

const PAGE_SIZE = 10;

export function MyBookingsPage() {
  const navigate = useNavigate();

  const [bookings, setBookings] = useState<BookingWithDetails[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);
  const [isLoading, setIsLoading] = useState(true);
  const [statusFilter, setStatusFilter] = useState<BookingStatus | undefined>();

  const [cancelDialogBooking, setCancelDialogBooking] = useState<BookingWithDetails | null>(null);
  const [isCancelling, setIsCancelling] = useState(false);

  useEffect(() => {
    const fetchBookings = async () => {
      setIsLoading(true);
      try {
        const response = await getMyBookings();
        let filtered = response.items;

        // Client-side filtering by status
        if (statusFilter) {
          filtered = filtered.filter((b) => b.status === statusFilter);
        }

        setBookings(filtered);
        setTotal(filtered.length);
      } catch (error) {
        console.error('Failed to fetch bookings:', error);
      } finally {
        setIsLoading(false);
      }
    };

    fetchBookings();
  }, [statusFilter]);

  const handleStatusFilter = (status?: BookingStatus) => {
    setStatusFilter(status);
    setPage(1);
  };

  const handleView = (id: string) => {
    navigate(`/bookings/${id}`);
  };

  const handleCancelClick = (id: string) => {
    const booking = bookings.find((b) => b.id === id);
    if (booking) {
      setCancelDialogBooking(booking);
    }
  };

  const handleCancelConfirm = async (reason?: string) => {
    if (!cancelDialogBooking) return;

    setIsCancelling(true);
    try {
      await cancelBooking(cancelDialogBooking.id, reason ? { reason } : undefined);
      // Refresh bookings
      const response = await getMyBookings();
      setBookings(response.items);
      setTotal(response.total);
      setCancelDialogBooking(null);
    } catch (error) {
      console.error('Failed to cancel booking:', error);
    } finally {
      setIsCancelling(false);
    }
  };

  return (
    <div className="max-w-4xl mx-auto px-4 py-8">
      <BookingList
        bookings={bookings}
        total={total}
        page={page}
        pageSize={PAGE_SIZE}
        isLoading={isLoading}
        isManager={false}
        title="My Bookings"
        onPageChange={setPage}
        onStatusFilter={handleStatusFilter}
        onView={handleView}
        onCancel={handleCancelClick}
      />

      {cancelDialogBooking && (
        <CancelBookingDialog
          bookingId={cancelDialogBooking.id}
          facilityName={cancelDialogBooking.facility_name}
          startTime={cancelDialogBooking.start_time}
          onConfirm={handleCancelConfirm}
          onCancel={() => setCancelDialogBooking(null)}
          isSubmitting={isCancelling}
        />
      )}
    </div>
  );
}
