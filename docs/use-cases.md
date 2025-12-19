# Property Management System - Use Cases

## Actors

- **Owner** - Apartment/unit owner who uses the system
- **Tenant** - Person renting a unit
- **Manager** - Property management company representative
- **Technical Manager** - Technical staff responsible for building maintenance

---

## UC-01: Notifications

### UC-01.1: Enable Push Notifications
**Actor:** Owner, Tenant
**Description:** User enables push notifications to receive alerts for new messages, events, or announcements.

---

## UC-02: Announcements

### UC-02.1: Create Announcement
**Actor:** Manager
**Description:** Manager creates a new announcement by entering text, optional attachments, and visibility settings.

### UC-02.2: Create Owners' Meeting Announcement
**Actor:** Manager
**Description:** Manager creates a special announcement for an owners' meeting, specifying the exact date and time.

### UC-02.3: Search Announcements
**Actor:** Owner, Tenant, Manager
**Description:** User searches announcements using full-text search by title or text.

### UC-02.4: Filter Announcements by Status
**Actor:** Owner, Tenant, Manager
**Description:** User filters announcements by status (published/draft/archived).

### UC-02.5: Filter Own Announcements
**Actor:** Manager
**Description:** Manager filters to display only announcements they created.

### UC-02.6: View Announcement Detail
**Actor:** Owner, Tenant, Manager
**Description:** User opens an announcement to view full content, attachments, comments, and metadata.

### UC-02.7: Comment on Announcement
**Actor:** Owner, Tenant, Manager
**Description:** User adds a comment to an announcement.

### UC-02.8: View Announcement Comments
**Actor:** Owner, Tenant, Manager
**Description:** User views existing comments on an announcement.

---

## UC-03: Faults

### UC-03.1: Report Fault
**Actor:** Owner, Tenant, Manager
**Description:** User reports a problem by entering title, description, location, and optionally attaching a photo.

### UC-03.2: Search Faults
**Actor:** Owner, Tenant, Manager
**Description:** User searches faults using the search field.

### UC-03.3: Filter Faults by Status
**Actor:** Owner, Tenant, Manager
**Description:** User filters faults by status (all/new/in progress/closed).

### UC-03.4: Filter Own Reported Faults
**Actor:** Owner, Tenant
**Description:** User filters to display only faults they reported.

### UC-03.5: View Fault Detail
**Actor:** Owner, Tenant, Manager
**Description:** User views fault details including resolution history and communication with manager.

### UC-03.6: Update Fault Status
**Actor:** Manager, Technical Manager
**Description:** Manager updates the resolution status of a fault.

### UC-03.7: Communicate on Fault
**Actor:** Owner, Tenant, Manager
**Description:** User adds communication/updates to a fault report.

---

## UC-04: Voting and Polls

### UC-04.1: Search Votes
**Actor:** Owner, Manager
**Description:** User searches votes using the search field.

### UC-04.2: Filter Votes by Status
**Actor:** Owner, Manager
**Description:** User filters votes by status (published/pending/completed).

### UC-04.3: View Vote Detail
**Actor:** Owner, Manager
**Description:** User views detailed wording of the question, results, and voter count.

### UC-04.4: Cast Vote
**Actor:** Owner
**Description:** Owner casts their vote on an ongoing poll before the deadline.

### UC-04.5: View Vote Results
**Actor:** Owner, Manager
**Description:** User views summary of results for a completed vote.

### UC-04.6: Comment on Vote
**Actor:** Owner, Manager
**Description:** User adds a comment to a vote.

### UC-04.7: Create Vote
**Actor:** Manager
**Description:** Manager creates a new vote/poll with question, options, and end date.

---

## UC-05: Messages

### UC-05.1: Create New Message
**Actor:** Owner, Tenant, Manager
**Description:** User creates a new conversation by selecting a recipient from the list of residents or managers.

### UC-05.2: Search Conversations
**Actor:** Owner, Tenant, Manager
**Description:** User searches existing conversations by person's name.

### UC-05.3: View Conversation List
**Actor:** Owner, Tenant, Manager
**Description:** User views list of conversations showing recipient info, last message date, and preview.

### UC-05.4: View Conversation Detail
**Actor:** Owner, Tenant, Manager
**Description:** User opens a conversation to view chronological transcript of messages.

### UC-05.5: Send Message
**Actor:** Owner, Tenant, Manager
**Description:** User sends a message within an existing conversation.

---

## UC-06: Neighbors

### UC-06.1: View Neighbors List
**Actor:** Owner, Tenant, Manager
**Description:** User views overview of registered owners and tenants in the building.

### UC-06.2: Invite Neighbor
**Actor:** Manager
**Description:** Manager sends an invitation to a neighbor to register in the application.

### UC-06.3: Search Neighbors
**Actor:** Owner, Tenant, Manager
**Description:** User searches neighbors by name.

### UC-06.4: Filter Neighbors by Entrance
**Actor:** Owner, Tenant, Manager
**Description:** User filters neighbors by entrance/building section.

### UC-06.5: Contact Neighbor
**Actor:** Owner, Tenant
**Description:** User initiates contact with a neighbor via the messaging system.

---

## UC-07: Contacts

### UC-07.1: View Manager Directory
**Actor:** Owner, Tenant
**Description:** User views directory of managers and technical managers with contact information.

### UC-07.2: View Manager Profile
**Actor:** Owner, Tenant
**Description:** User views detailed profile of a manager.

### UC-07.3: Contact Manager
**Actor:** Owner, Tenant
**Description:** User contacts a manager via email or messaging system.

---

## UC-08: Documents

### UC-08.1: Search Documents
**Actor:** Owner, Tenant, Manager
**Description:** User searches for files or folders by name.

### UC-08.2: Browse Document Folders
**Actor:** Owner, Tenant, Manager
**Description:** User navigates through folder structure to find documents.

### UC-08.3: Download Document
**Actor:** Owner, Tenant, Manager
**Description:** User downloads a document from the archive.

### UC-08.4: Forward Document
**Actor:** Owner, Tenant, Manager
**Description:** User forwards a document to another person.

### UC-08.5: View Document Version History
**Actor:** Owner, Tenant, Manager
**Description:** User views the version history of a document.

### UC-08.6: Upload Document
**Actor:** Manager
**Description:** Manager uploads a new document to the archive.

### UC-08.7: Create Document Folder
**Actor:** Manager
**Description:** Manager creates a new folder for organizing documents.

---

## UC-09: Forms

### UC-09.1: Search Forms
**Actor:** Owner, Tenant
**Description:** User searches forms by name.

### UC-09.2: Download Form
**Actor:** Owner, Tenant
**Description:** User downloads a form for printing or digital use.

### UC-09.3: Publish Form
**Actor:** Manager
**Description:** Manager publishes a form for residents to access.

---

## UC-10: Person-Months

### UC-10.1: Add Person-Month Record
**Actor:** Owner, Manager
**Description:** User records the number of persons living in a residential unit for a given month.

### UC-10.2: View Person-Month History
**Actor:** Owner, Manager
**Description:** User views historical person-month records for a unit.

---

## UC-11: Self-Readings

### UC-11.1: Submit Meter Reading
**Actor:** Owner
**Description:** Owner enters the current meter reading, optionally attaching a photo of the meter.

### UC-11.2: View Self-Readings Overview
**Actor:** Manager
**Description:** Manager views overview of all submitted self-readings in a table format.

### UC-11.3: Export Self-Readings
**Actor:** Manager
**Description:** Manager exports self-readings data for external processing.

### UC-11.4: Verify Meter Reading
**Actor:** Manager
**Description:** Manager verifies a submitted meter reading.

---

## UC-12: Outages

### UC-12.1: View Outages List
**Actor:** Owner, Tenant
**Description:** User views list of current and planned water and electricity outages.

### UC-12.2: View Outages by Commodity
**Actor:** Owner, Tenant
**Description:** User views outages filtered by commodity type (water/electricity).

### UC-12.3: Call Supplier
**Actor:** Owner, Tenant
**Description:** User initiates a phone call to a supplier directly from the app.

### UC-12.4: View Supplier Outage Page
**Actor:** Owner, Tenant
**Description:** User is redirected to supplier's official page for detailed outage information.

---

## UC-13: News

### UC-13.1: Search News
**Actor:** Owner, Tenant, Manager
**Description:** User searches news articles by keywords.

### UC-13.2: View News Article
**Actor:** Owner, Tenant, Manager
**Description:** User views a news article with headline, description, and attachments.

### UC-13.3: React to News Article
**Actor:** Owner, Tenant
**Description:** User adds a reaction (like) to a news article.

### UC-13.4: Publish News Article
**Actor:** Manager
**Description:** Manager publishes a new news article with headline, description, and optional attachments.
