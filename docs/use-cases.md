# Property Management System - Use Cases

## Actors

- **Owner** - Apartment/unit owner who uses the system
- **Tenant** - Person renting a unit
- **Manager** - Property management company representative
- **Technical Manager** - Technical staff responsible for building maintenance
- **System Administrator** - User with full system access for configuration and maintenance

---

## UC-01: Notifications

### UC-01.1: Enable Push Notifications
**Actor:** Owner, Tenant
**Description:** User enables push notifications to receive alerts for new messages, events, or announcements.

### UC-01.2: Disable Push Notifications
**Actor:** Owner, Tenant
**Description:** User disables push notifications to stop receiving alerts.

### UC-01.3: Configure Notification Preferences
**Actor:** Owner, Tenant, Manager
**Description:** User configures which types of notifications they want to receive (messages, announcements, faults, votes, etc.).

### UC-01.4: View Notification History
**Actor:** Owner, Tenant, Manager
**Description:** User views a chronological list of all received notifications.

### UC-01.5: Mark Notification as Read
**Actor:** Owner, Tenant, Manager
**Description:** User marks a specific notification as read.

### UC-01.6: Mark All Notifications as Read
**Actor:** Owner, Tenant, Manager
**Description:** User marks all unread notifications as read in one action.

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

### UC-02.9: Edit Announcement
**Actor:** Manager
**Description:** Manager edits an existing announcement to update text, attachments, or visibility settings.

### UC-02.10: Delete Announcement
**Actor:** Manager
**Description:** Manager permanently deletes an announcement from the system.

### UC-02.11: Archive Announcement
**Actor:** Manager
**Description:** Manager archives an announcement to remove it from the active list while preserving it for historical reference.

### UC-02.12: Pin Announcement to Top
**Actor:** Manager
**Description:** Manager pins an important announcement to the top of the announcements list.

### UC-02.13: Schedule Announcement Publication
**Actor:** Manager
**Description:** Manager schedules an announcement to be automatically published at a future date and time.

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

### UC-03.8: Assign Fault to Technical Manager
**Actor:** Manager
**Description:** Manager assigns a reported fault to a specific technical manager for resolution.

### UC-03.9: Set Fault Priority
**Actor:** Manager, Technical Manager
**Description:** Manager sets the priority level (low/medium/high/critical) for a fault report.

### UC-03.10: Close Fault
**Actor:** Manager, Technical Manager
**Description:** Manager closes a fault after it has been successfully resolved.

### UC-03.11: Reopen Fault
**Actor:** Owner, Tenant, Manager
**Description:** User reopens a previously closed fault if the issue persists or recurs.

### UC-03.12: Escalate Fault
**Actor:** Manager, Technical Manager
**Description:** Manager escalates a fault to higher priority or management level.

### UC-03.13: Add Photo to Existing Fault
**Actor:** Owner, Tenant, Manager, Technical Manager
**Description:** User adds additional photos to an existing fault report to document progress or additional issues.

### UC-03.14: Request Fault Update
**Actor:** Owner, Tenant
**Description:** User requests an update on the status of their reported fault.

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

### UC-04.8: Edit Vote
**Actor:** Manager
**Description:** Manager edits a vote before it is published (question, options, end date).

### UC-04.9: Cancel Vote
**Actor:** Manager
**Description:** Manager cancels an ongoing or pending vote.

### UC-04.10: Extend Voting Deadline
**Actor:** Manager
**Description:** Manager extends the deadline for an ongoing vote to allow more time for participation.

### UC-04.11: Delegate Vote (Proxy Voting)
**Actor:** Owner
**Description:** Owner delegates their voting rights to another owner for a specific vote.

### UC-04.12: Send Vote Reminder
**Actor:** Manager
**Description:** Manager sends a reminder notification to owners who have not yet voted.

### UC-04.13: Export Vote Results
**Actor:** Manager
**Description:** Manager exports vote results to PDF or spreadsheet format.

### UC-04.14: Change Vote
**Actor:** Owner
**Description:** Owner changes their previously cast vote (if allowed by vote settings).

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

### UC-05.6: Delete Message
**Actor:** Owner, Tenant, Manager
**Description:** User deletes a message they sent from the conversation.

### UC-05.7: Delete Conversation
**Actor:** Owner, Tenant, Manager
**Description:** User deletes an entire conversation from their inbox.

### UC-05.8: Create Group Conversation
**Actor:** Manager
**Description:** Manager creates a group conversation with multiple recipients.

### UC-05.9: Attach File to Message
**Actor:** Owner, Tenant, Manager
**Description:** User attaches a file (document, image) to a message.

### UC-05.10: View Read Receipt
**Actor:** Owner, Tenant, Manager
**Description:** User views whether their sent message has been read by the recipient.

### UC-05.11: Archive Conversation
**Actor:** Owner, Tenant, Manager
**Description:** User archives a conversation to remove it from active inbox while preserving it.

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

### UC-06.6: Edit Neighbor Information
**Actor:** Manager
**Description:** Manager edits neighbor information (unit assignment, contact details).

### UC-06.7: Remove Neighbor from Building
**Actor:** Manager
**Description:** Manager removes a neighbor from the building's resident list.

### UC-06.8: Resend Invitation
**Actor:** Manager
**Description:** Manager resends an invitation to a neighbor who has not yet registered.

### UC-06.9: Cancel Pending Invitation
**Actor:** Manager
**Description:** Manager cancels a pending invitation that has not been accepted.

### UC-06.10: View Invitation Status
**Actor:** Manager
**Description:** Manager views the status of sent invitations (pending, accepted, expired).

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

### UC-07.4: Add Manager Contact
**Actor:** Manager, System Administrator
**Description:** Administrator adds a new manager or technical manager to the contacts directory.

### UC-07.5: Edit Manager Contact
**Actor:** Manager, System Administrator
**Description:** Administrator edits manager contact information (role, email, phone).

### UC-07.6: Remove Manager Contact
**Actor:** Manager, System Administrator
**Description:** Administrator removes a manager from the contacts directory.

### UC-07.7: Set Primary Contact for Building
**Actor:** Manager
**Description:** Manager designates a primary contact person for a specific building.

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

### UC-08.8: Delete Document
**Actor:** Manager
**Description:** Manager permanently deletes a document from the archive.

### UC-08.9: Move Document to Folder
**Actor:** Manager
**Description:** Manager moves a document to a different folder.

### UC-08.10: Rename Document
**Actor:** Manager
**Description:** Manager renames a document in the archive.

### UC-08.11: Rename Folder
**Actor:** Manager
**Description:** Manager renames an existing folder.

### UC-08.12: Delete Folder
**Actor:** Manager
**Description:** Manager deletes a folder (must be empty or contents moved).

### UC-08.13: Set Document Access Permissions
**Actor:** Manager
**Description:** Manager sets access permissions for a document (who can view/download).

### UC-08.14: Share Document with Specific Owners
**Actor:** Manager
**Description:** Manager shares a document with specific owners rather than all residents.

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

### UC-09.4: Submit Filled Form Online
**Actor:** Owner, Tenant
**Description:** User fills out and submits a form directly through the application.

### UC-09.5: Edit Form Template
**Actor:** Manager
**Description:** Manager edits an existing form template.

### UC-09.6: Delete Form
**Actor:** Manager
**Description:** Manager deletes a form from the system.

### UC-09.7: View Form Submissions
**Actor:** Manager
**Description:** Manager views all submissions for a specific form.

### UC-09.8: Export Form Submissions
**Actor:** Manager
**Description:** Manager exports form submissions to spreadsheet format.

---

## UC-10: Person-Months

### UC-10.1: Add Person-Month Record
**Actor:** Owner, Manager
**Description:** User records the number of persons living in a residential unit for a given month.

### UC-10.2: View Person-Month History
**Actor:** Owner, Manager
**Description:** User views historical person-month records for a unit.

### UC-10.3: Edit Person-Month Record
**Actor:** Owner, Manager
**Description:** User edits an existing person-month record to correct errors.

### UC-10.4: Delete Person-Month Record
**Actor:** Manager
**Description:** Manager deletes an incorrect person-month record.

### UC-10.5: Bulk Entry Person-Months
**Actor:** Manager
**Description:** Manager enters person-month data for multiple units at once.

### UC-10.6: Export Person-Month Data
**Actor:** Manager
**Description:** Manager exports person-month data for external processing or reporting.

### UC-10.7: Set Reminder for Person-Month Entry
**Actor:** Manager
**Description:** Manager sets up automatic reminders for owners to submit person-month data.

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

### UC-11.5: Edit Meter Reading
**Actor:** Owner, Manager
**Description:** User edits a previously submitted meter reading before verification.

### UC-11.6: Reject Meter Reading
**Actor:** Manager
**Description:** Manager rejects a submitted meter reading that appears incorrect.

### UC-11.7: Request Reading Correction
**Actor:** Manager
**Description:** Manager requests the owner to resubmit a corrected meter reading.

### UC-11.8: Send Reading Submission Reminder
**Actor:** Manager
**Description:** Manager sends reminders to owners who have not submitted their meter readings.

### UC-11.9: View Reading History
**Actor:** Owner, Manager
**Description:** User views historical meter readings for a unit.

### UC-11.10: Compare Readings Over Time
**Actor:** Manager
**Description:** Manager compares meter readings over time to identify anomalies or consumption patterns.

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

### UC-12.5: Report Unplanned Outage
**Actor:** Owner, Tenant
**Description:** User reports an unplanned outage (water, electricity, heating) affecting the building.

### UC-12.6: Subscribe to Outage Notifications
**Actor:** Owner, Tenant
**Description:** User subscribes to receive notifications about outages in their area.

### UC-12.7: View Outage History
**Actor:** Owner, Tenant, Manager
**Description:** User views historical outages for the building or area.

### UC-12.8: Add Outage
**Actor:** Manager
**Description:** Manager manually adds an outage notice to inform residents.

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

### UC-13.5: Edit News Article
**Actor:** Manager
**Description:** Manager edits an existing news article.

### UC-13.6: Delete News Article
**Actor:** Manager
**Description:** Manager permanently deletes a news article.

### UC-13.7: Archive News Article
**Actor:** Manager
**Description:** Manager archives an old news article to remove it from the main feed.

### UC-13.8: Comment on News Article
**Actor:** Owner, Tenant, Manager
**Description:** User adds a comment to a news article.

### UC-13.9: Share News Article
**Actor:** Owner, Tenant
**Description:** User shares a news article via external channels (email, social media link).

---

## UC-14: User Account Management

### UC-14.1: Register Account
**Actor:** Owner, Tenant
**Description:** New user registers an account using an invitation link or registration code.

### UC-14.2: Login
**Actor:** Owner, Tenant, Manager, Technical Manager, System Administrator
**Description:** User logs into the system using their credentials.

### UC-14.3: Logout
**Actor:** Owner, Tenant, Manager, Technical Manager, System Administrator
**Description:** User logs out of the system.

### UC-14.4: Reset Password
**Actor:** Owner, Tenant, Manager, Technical Manager
**Description:** User requests a password reset via email.

### UC-14.5: Change Password
**Actor:** Owner, Tenant, Manager, Technical Manager
**Description:** User changes their current password.

### UC-14.6: Update Profile Information
**Actor:** Owner, Tenant, Manager, Technical Manager
**Description:** User updates their profile information (name, phone, email).

### UC-14.7: Upload Profile Photo
**Actor:** Owner, Tenant, Manager, Technical Manager
**Description:** User uploads or changes their profile photo.

### UC-14.8: Delete Account
**Actor:** Owner, Tenant
**Description:** User requests deletion of their account and personal data.

### UC-14.9: View Own Activity History
**Actor:** Owner, Tenant, Manager
**Description:** User views their own activity history in the system.

---

## UC-15: Building/Property Management

### UC-15.1: Add Building
**Actor:** Manager, System Administrator
**Description:** Administrator adds a new building to the system.

### UC-15.2: Edit Building Information
**Actor:** Manager, System Administrator
**Description:** Administrator edits building information (address, name, details).

### UC-15.3: View Building Details
**Actor:** Owner, Tenant, Manager
**Description:** User views detailed information about a building.

### UC-15.4: Add Unit to Building
**Actor:** Manager, System Administrator
**Description:** Administrator adds a new unit/apartment to a building.

### UC-15.5: Edit Unit Information
**Actor:** Manager, System Administrator
**Description:** Administrator edits unit information (floor, size, type).

### UC-15.6: Assign Owner to Unit
**Actor:** Manager, System Administrator
**Description:** Administrator assigns an owner to a specific unit.

### UC-15.7: View Building Statistics
**Actor:** Manager
**Description:** Manager views statistics for a building (occupancy, faults, payments).

---

## UC-16: Financial Management

### UC-16.1: View Account Balance
**Actor:** Owner
**Description:** Owner views their current account balance and outstanding payments.

### UC-16.2: View Payment History
**Actor:** Owner, Manager
**Description:** User views historical payment records.

### UC-16.3: Make Payment
**Actor:** Owner
**Description:** Owner makes a payment through the system (if online payments are enabled).

### UC-16.4: Generate Invoice
**Actor:** Manager
**Description:** Manager generates an invoice for an owner.

### UC-16.5: Export Financial Report
**Actor:** Manager
**Description:** Manager exports financial reports for a building or time period.

### UC-16.6: View Annual Settlement
**Actor:** Owner
**Description:** Owner views their annual settlement/statement of costs.

### UC-16.7: Download Invoice PDF
**Actor:** Owner
**Description:** Owner downloads an invoice as a PDF document.

---

## UC-17: Reports and Analytics

### UC-17.1: Generate Fault Statistics Report
**Actor:** Manager
**Description:** Manager generates a report on fault statistics (types, resolution times, trends).

### UC-17.2: Generate Voting Participation Report
**Actor:** Manager
**Description:** Manager generates a report on voting participation rates.

### UC-17.3: Generate Occupancy Report
**Actor:** Manager
**Description:** Manager generates a report on building occupancy based on person-month data.

### UC-17.4: Generate Consumption Report
**Actor:** Manager
**Description:** Manager generates a report on utility consumption based on meter readings.

### UC-17.5: Export Report to PDF/Excel
**Actor:** Manager
**Description:** Manager exports any generated report to PDF or Excel format.

---

## UC-18: System Administration

### UC-18.1: Manage User Roles
**Actor:** System Administrator
**Description:** Administrator assigns or modifies user roles and permissions.

### UC-18.2: View Audit Log
**Actor:** System Administrator
**Description:** Administrator views system audit log of all user actions.

### UC-18.3: Configure System Settings
**Actor:** System Administrator
**Description:** Administrator configures system-wide settings (email templates, defaults).

### UC-18.4: Manage Email Templates
**Actor:** System Administrator
**Description:** Administrator customizes email notification templates.

### UC-18.5: Backup Data
**Actor:** System Administrator
**Description:** Administrator initiates or schedules data backups.

### UC-18.6: View System Statistics
**Actor:** System Administrator
**Description:** Administrator views system-wide statistics (users, buildings, activity).

---

## Summary

| Category | Use Cases |
|----------|-----------|
| UC-01: Notifications | 6 |
| UC-02: Announcements | 13 |
| UC-03: Faults | 14 |
| UC-04: Voting and Polls | 14 |
| UC-05: Messages | 11 |
| UC-06: Neighbors | 10 |
| UC-07: Contacts | 7 |
| UC-08: Documents | 14 |
| UC-09: Forms | 8 |
| UC-10: Person-Months | 7 |
| UC-11: Self-Readings | 10 |
| UC-12: Outages | 8 |
| UC-13: News | 9 |
| UC-14: User Account Management | 9 |
| UC-15: Building/Property Management | 7 |
| UC-16: Financial Management | 7 |
| UC-17: Reports and Analytics | 5 |
| UC-18: System Administration | 6 |
| **TOTAL** | **152** |
