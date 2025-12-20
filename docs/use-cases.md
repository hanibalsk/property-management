# Property Management System - Use Cases

## Actors

### Platform Level
- **Super Administrator** - Global platform administrator managing all organizations
- **AI System** - Automated AI/ML system performing intelligent operations
- **IoT Device** - Smart building devices and sensors

### Organization Level
- **Organization Admin** - Administrator of a housing cooperative or property management company
- **Manager** - Property management company representative managing buildings
- **Technical Manager** - Technical staff responsible for building maintenance

### Unit Level
- **Owner** - Apartment/unit owner who uses the system
- **Owner Delegate** - Person with delegated rights from owner (voting, payments, etc.)
- **Tenant** - Person renting a unit under lease agreement
- **Resident** - Person living in unit without ownership (family member, roommate)
- **Property Manager** - Short-term rental manager (Airbnb/Booking administrator)
- **Guest** - Temporary visitor or short-term rental guest

### External Actors
- **Real Estate Agent** - Licensed real estate broker managing listings and tenant screening
- **Real Estate Portal** - External listing portal system (API integration)
- **System Administrator** - User with full system access for configuration and maintenance

### Reality Portal Level
- **Portal User** - Regular user browsing and searching property listings on Reality Portal
- **Realtor** - Licensed real estate agent listing and managing properties on portal
- **Agency Manager** - Manages realtors within a reality agency
- **Agency Owner** - Owns and administers a reality agency organization

### Actor Hierarchy
```
Super Administrator
├── Organization (Housing Cooperative / Property Management Company)
│       ├── Organization Admin
│       ├── Manager
│       ├── Technical Manager
│       └── Building
│               └── Unit
│                       ├── Owner
│                       │       └── Owner Delegate
│                       ├── Tenant
│                       ├── Resident
│                       ├── Property Manager
│                       │       └── Guest
│                       └── Real Estate Agent
│
└── Reality Portal
        ├── Portal User
        └── Agency (Reality Agency)
                ├── Agency Owner
                ├── Agency Manager
                └── Realtor
```

---

## UC-01: Notifications (ppt-web, mobile)

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

## UC-02: Announcements (ppt-web, mobile)

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

## UC-03: Faults (ppt-web, mobile)

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

## UC-04: Voting and Polls (ppt-web, mobile)

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

## UC-05: Messages (ppt-web, mobile)

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

## UC-06: Neighbors (ppt-web, mobile)

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

## UC-07: Contacts (ppt-web, mobile)

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

## UC-08: Documents (ppt-web, mobile)

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

## UC-09: Forms (ppt-web, mobile)

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

## UC-10: Person-Months (ppt-web, mobile)

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

## UC-11: Self-Readings (ppt-web, mobile)

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

## UC-12: Outages (ppt-web, mobile)

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

## UC-13: News (ppt-web, mobile)

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

## UC-14: User Account Management (ppt-web, mobile, reality-web, mobile-native)

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

### UC-14.10: Setup Multi-Factor Authentication
**Actor:** Owner, Tenant, Manager, Technical Manager
**Description:** User sets up MFA using TOTP authenticator app or SMS verification during initial configuration.

### UC-14.11: Login with SSO
**Actor:** Manager, Technical Manager, Organization Admin
**Description:** User authenticates using organization's SAML/OIDC identity provider for single sign-on.

### UC-14.12: Handle Account Lockout
**Actor:** Owner, Tenant, Manager, Technical Manager
**Description:** User is locked out after multiple failed login attempts and must wait or contact support to unlock.

---

## UC-15: Building/Property Management (ppt-web, mobile)

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

### UC-15.8: Bulk Import Buildings
**Actor:** System Administrator, Organization Admin
**Description:** Administrator imports multiple buildings from a CSV file with address and configuration data.

### UC-15.9: Merge Duplicate Buildings
**Actor:** System Administrator
**Description:** Administrator merges two duplicate building records, consolidating units and history.

### UC-15.10: Archive Building
**Actor:** Manager, System Administrator
**Description:** Administrator archives a building that is no longer actively managed while preserving historical data.

---

## UC-16: Financial Management (ppt-web, mobile)

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

### UC-16.8: Reconcile Payments
**Actor:** Manager
**Description:** Manager reconciles bank payments with invoices and marks them as paid.

### UC-16.9: Process Refund
**Actor:** Manager
**Description:** Manager processes a refund for an overpayment or cancelled service.

### UC-16.10: Configure Late Payment Fees
**Actor:** Manager, Organization Admin
**Description:** Administrator configures automatic late payment fee rules and percentages.

---

## UC-17: Reports and Analytics (ppt-web)

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

## UC-18: System Administration (ppt-web)

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

## UC-19: Real-time & Mobile Features (ppt-web, mobile, reality-web, mobile-native)

### UC-19.1: Real-time Fault Status Updates
**Actor:** Owner, Tenant, Manager
**Description:** User receives live updates when fault status changes without refreshing the page.

### UC-19.2: Live Voting Results
**Actor:** Owner, Manager
**Description:** User views real-time vote count updates during active polls.

### UC-19.3: Typing Indicators in Messages
**Actor:** Owner, Tenant, Manager
**Description:** User sees when the other person is typing a message in a conversation.

### UC-19.4: Presence Indicators
**Actor:** Owner, Tenant
**Description:** User sees online/offline/away status of managers in the contacts list.

### UC-19.5: Live Document Collaboration
**Actor:** Manager
**Description:** Multiple managers can edit a document simultaneously with real-time sync.

### UC-19.6: Offline Mode Support
**Actor:** Owner, Tenant, Manager
**Description:** User accesses cached data and queued actions when internet is unavailable.

### UC-19.7: Background Sync
**Actor:** Owner, Tenant, Manager
**Description:** System automatically syncs queued actions when internet connection is restored.

### UC-19.8: Low Bandwidth Mode
**Actor:** Owner, Tenant
**Description:** User enables compressed images and reduced data transfer for poor connections.

### UC-19.9: Progressive Web App Installation
**Actor:** Owner, Tenant, Manager
**Description:** User installs the application as a native-like app on their device.

### UC-19.10: Switch Application Language
**Actor:** Owner, Tenant, Manager
**Description:** User changes the application interface language.

### UC-19.11: Auto-translate Announcements
**Actor:** AI System
**Description:** AI automatically translates announcements to the user's preferred language.

### UC-19.12: Multi-language Document Support
**Actor:** Manager
**Description:** Manager uploads documents in multiple languages with automatic language detection.

---

## UC-20: AI/ML Features (ppt-web, mobile, reality-web, mobile-native)

### UC-20.1: AI Chatbot for Common Questions
**Actor:** Owner, Tenant, AI System
**Description:** User asks common questions about rules, payments, contacts and receives instant AI-powered answers.

### UC-20.2: AI-Assisted Fault Reporting
**Actor:** Owner, Tenant, AI System
**Description:** AI chatbot guides users through fault reporting with smart questions to gather complete information.

### UC-20.3: AI Fault Categorization
**Actor:** AI System
**Description:** AI automatically categorizes reported faults based on description and assigns appropriate tags.

### UC-20.4: AI Response Suggestions
**Actor:** Manager, AI System
**Description:** AI suggests response templates to managers for common queries and complaints.

### UC-20.5: Virtual Building Assistant
**Actor:** Owner, Tenant, AI System
**Description:** User interacts with voice-activated assistant for hands-free building information access.

### UC-20.6: Predict Maintenance Needs
**Actor:** Manager, AI System
**Description:** AI predicts when building equipment will need maintenance based on fault history and patterns.

### UC-20.7: Consumption Anomaly Detection
**Actor:** Manager, AI System
**Description:** AI detects unusual water/electricity consumption patterns and alerts managers.

### UC-20.8: Predict Payment Delays
**Actor:** Manager, AI System
**Description:** AI identifies owners likely to have payment issues based on historical patterns.

### UC-20.9: Fault Resolution Time Prediction
**Actor:** Owner, Tenant, AI System
**Description:** AI estimates fault resolution time based on fault type and historical data.

### UC-20.10: Occupancy Prediction
**Actor:** Manager, AI System
**Description:** AI predicts future building occupancy trends based on historical data.

### UC-20.11: OCR Meter Reading
**Actor:** Owner, AI System
**Description:** AI automatically extracts meter reading value from uploaded photo of the meter.

### UC-20.12: AI Fault Image Analysis
**Actor:** AI System
**Description:** AI analyzes fault photos to assess severity, type, and urgency of the issue.

### UC-20.13: Document OCR & Indexing
**Actor:** Manager, AI System
**Description:** AI extracts text from uploaded documents to enable full-text search.

### UC-20.14: Face Recognition for Access
**Actor:** Owner, Tenant, AI System
**Description:** User authenticates using facial recognition for building access integration.

### UC-20.15: Damage Assessment from Photos
**Actor:** Manager, AI System
**Description:** AI estimates repair costs and severity from damage photos.

### UC-20.16: Sentiment Analysis on Feedback
**Actor:** Manager, AI System
**Description:** AI analyzes resident sentiment from messages and comments to identify satisfaction trends.

### UC-20.17: Smart Search with NLP
**Actor:** Owner, Tenant, Manager, AI System
**Description:** User searches using natural language queries across all content types.

### UC-20.18: Auto-summarize Long Documents
**Actor:** Owner, Tenant, AI System
**Description:** AI generates concise summaries of lengthy documents for quick review.

### UC-20.19: Auto-generate Meeting Minutes
**Actor:** Manager, AI System
**Description:** AI summarizes owners' meeting discussions and generates structured minutes.

### UC-20.20: Spam/Abuse Detection
**Actor:** AI System
**Description:** AI automatically filters inappropriate content and spam in messages and comments.

### UC-20.21: Recommend Similar Faults
**Actor:** Manager, Technical Manager, AI System
**Description:** AI shows similar past faults and their resolutions to help resolve current issues.

### UC-20.22: Suggest Document Tags
**Actor:** Manager, AI System
**Description:** AI suggests relevant tags for uploaded documents based on content analysis.

### UC-20.23: Personalized News Feed
**Actor:** Owner, Tenant, AI System
**Description:** AI curates news and announcements based on user interests and reading history.

### UC-20.24: Smart Notification Prioritization
**Actor:** AI System
**Description:** AI prioritizes notifications by importance and user preferences to reduce noise.

---

## UC-21: IoT & Smart Building (ppt-web, mobile)

### UC-21.1: Connect Smart Meters
**Actor:** Manager, System Administrator, IoT Device
**Description:** Administrator integrates IoT smart meters for automatic reading collection.

### UC-21.2: Smart Lock Integration
**Actor:** Owner, Tenant, Manager, IoT Device
**Description:** User manages building access through integrated smart locks.

### UC-21.3: Environmental Sensors Dashboard
**Actor:** Owner, Tenant, Manager, IoT Device
**Description:** User views real-time temperature, humidity, and air quality data from sensors.

### UC-21.4: Elevator Status Monitoring
**Actor:** Manager, Technical Manager, IoT Device
**Description:** Manager monitors real-time elevator status and receives fault alerts.

### UC-21.5: Parking Sensor Integration
**Actor:** Owner, Tenant, IoT Device
**Description:** User views available parking spots through integrated parking sensors.

### UC-21.6: CCTV Access
**Actor:** Manager, System Administrator, IoT Device
**Description:** Authorized user views building security camera feeds within the application.

### UC-21.7: Fire/Smoke Alarm Integration
**Actor:** Owner, Tenant, Manager, IoT Device
**Description:** User receives instant push notifications from integrated fire/smoke detection systems.

### UC-21.8: Energy Management Dashboard
**Actor:** Manager, IoT Device
**Description:** Manager monitors and analyzes building-wide energy consumption data.

### UC-21.9: Water Leak Detection
**Actor:** Manager, Technical Manager, IoT Device
**Description:** System sends instant alerts when water leak sensors detect a leak.

### UC-21.10: Automated Climate Control
**Actor:** Manager, IoT Device
**Description:** Manager configures and monitors automated HVAC for common areas.

---

## UC-22: External Integrations (ppt-web, mobile)

### UC-22.1: Calendar Integration
**Actor:** Owner, Tenant, Manager
**Description:** User syncs meetings, events, and deadlines with Google/Outlook calendar.

### UC-22.2: Accounting System Integration
**Actor:** Manager, System Administrator
**Description:** Manager syncs financial data with external accounting software.

### UC-22.3: Government Portal Integration
**Actor:** Manager
**Description:** Manager submits regulatory reports electronically to government portals.

### UC-22.4: Bank Payment Integration
**Actor:** Owner
**Description:** Owner makes direct bank transfers for payments through integrated banking.

### UC-22.5: SMS Gateway Integration
**Actor:** Manager, System Administrator
**Description:** System sends SMS notifications for critical alerts via integrated SMS gateway.

### UC-22.6: Email Service Integration
**Actor:** Manager, System Administrator
**Description:** Administrator configures advanced email delivery with tracking and analytics.

### UC-22.7: Document Signing Integration
**Actor:** Owner, Manager
**Description:** User electronically signs documents using integrated e-signature service (DocuSign, etc.).

### UC-22.8: Video Conferencing Integration
**Actor:** Owner, Manager
**Description:** Manager conducts virtual owners' meetings via integrated Zoom/Teams.

### UC-22.9: Public API for Third Parties
**Actor:** System Administrator
**Description:** Administrator manages API access for external service providers.

### UC-22.10: Webhook Notifications
**Actor:** System Administrator
**Description:** Administrator configures real-time webhooks to notify external systems of events.

---

## UC-23: Security & Compliance (ppt-web, mobile, reality-web, mobile-native)

### UC-23.1: Two-Factor Authentication
**Actor:** Owner, Tenant, Manager, Technical Manager
**Description:** User enables 2FA for enhanced account security.

### UC-23.2: Biometric Login
**Actor:** Owner, Tenant, Manager
**Description:** User authenticates using fingerprint or Face ID on supported devices.

### UC-23.3: Session Management
**Actor:** Owner, Tenant, Manager
**Description:** User views active sessions and can terminate sessions on other devices.

### UC-23.4: GDPR Data Export
**Actor:** Owner, Tenant
**Description:** User exports all their personal data in a portable format (GDPR compliance).

### UC-23.5: GDPR Data Deletion
**Actor:** Owner, Tenant
**Description:** User requests complete deletion of their personal data from the system.

### UC-23.6: Privacy Settings Management
**Actor:** Owner, Tenant
**Description:** User controls what personal information is visible to other residents.

### UC-23.7: Consent Management
**Actor:** Owner, Tenant
**Description:** User manages consent for various data processing activities.

### UC-23.8: Document Encryption
**Actor:** Manager
**Description:** Manager uploads documents with end-to-end encryption for sensitive content.

### UC-23.9: Secure Document Viewer
**Actor:** Owner, Tenant
**Description:** User views sensitive documents in a secure viewer that prevents downloading.

### UC-23.10: Audit Trail for Sensitive Actions
**Actor:** System Administrator
**Description:** Administrator views detailed audit logs of all security-relevant actions.

### UC-23.11: View Rate Limiting Status
**Actor:** System Administrator
**Description:** Administrator views rate limiting alerts and notifications when API limits are approached.

### UC-23.12: Manage Trusted Devices
**Actor:** Owner, Tenant, Manager
**Description:** User views and removes trusted devices that are allowed to bypass MFA verification.

---

## UC-24: Community & Social (ppt-web, mobile)

### UC-24.1: Community Forum
**Actor:** Owner, Tenant, Manager
**Description:** User participates in discussion boards organized by topics.

### UC-24.2: Marketplace/Classifieds
**Actor:** Owner, Tenant
**Description:** User posts and browses items for sale/trade within the building community.

### UC-24.3: Event Calendar
**Actor:** Owner, Tenant, Manager
**Description:** User views and registers for community events and activities.

### UC-24.4: Facility Booking
**Actor:** Owner, Tenant
**Description:** User reserves common areas (party room, gym, meeting room, etc.).

### UC-24.5: Neighbor Recommendations
**Actor:** Owner, Tenant
**Description:** User requests or offers services (babysitting, pet care, etc.) among neighbors.

### UC-24.6: Pet Registry
**Actor:** Owner, Tenant, Manager
**Description:** User registers pets living in their unit for building records.

### UC-24.7: Vehicle Registry
**Actor:** Owner, Tenant, Manager
**Description:** User registers vehicles for parking management and identification.

### UC-24.8: Package Tracking
**Actor:** Owner, Tenant
**Description:** User tracks incoming packages and receives notifications upon delivery.

### UC-24.9: Visitor Management
**Actor:** Owner, Tenant
**Description:** User pre-registers visitors and generates temporary access codes.

### UC-24.10: Emergency Contact Directory
**Actor:** Owner, Tenant, Manager
**Description:** User accesses emergency contacts (police, fire, ambulance, building emergency).

---

## UC-25: Accessibility (ppt-web, mobile, reality-web, mobile-native)

### UC-25.1: Screen Reader Compatibility
**Actor:** Owner, Tenant, Manager
**Description:** User navigates the application using screen reader software (WCAG compliance).

### UC-25.2: Voice Navigation
**Actor:** Owner, Tenant, Manager
**Description:** User navigates and controls the app using voice commands.

### UC-25.3: High Contrast Mode
**Actor:** Owner, Tenant, Manager
**Description:** User enables high contrast display mode for improved visibility.

### UC-25.4: Text Size Adjustment
**Actor:** Owner, Tenant, Manager
**Description:** User adjusts text size throughout the application for better readability.

### UC-25.5: Video Captions
**Actor:** Owner, Tenant, Manager
**Description:** User views captions on all video content within the application.

### UC-25.6: Sign Language Support
**Actor:** Owner, Tenant, Manager
**Description:** User accesses sign language video versions of important announcements.

### UC-25.7: Simplified Interface Mode
**Actor:** Owner, Tenant
**Description:** User enables a simplified interface with reduced complexity for easier navigation.

### UC-25.8: Keyboard Navigation
**Actor:** Owner, Tenant, Manager
**Description:** User navigates the entire application using only keyboard controls.

---

## UC-26: Workflow Automation (ppt-web, mobile)

### UC-26.1: Automated Fault Routing
**Actor:** AI System, Manager
**Description:** System automatically assigns faults to appropriate technicians based on type and location.

### UC-26.2: Automated Payment Reminders
**Actor:** AI System, Owner
**Description:** System sends scheduled payment reminder emails before due dates.

### UC-26.3: Automated Meter Reading Reminders
**Actor:** AI System, Owner
**Description:** System sends reminders to owners before meter reading submission deadlines.

### UC-26.4: Automated Report Generation
**Actor:** Manager, AI System
**Description:** Manager schedules automatic generation and delivery of regular reports.

### UC-26.5: Workflow Templates
**Actor:** Manager, System Administrator
**Description:** Administrator creates reusable workflow templates for common processes.

### UC-26.6: Approval Workflows
**Actor:** Manager
**Description:** Manager creates multi-step approval workflows for documents and expenses.

### UC-26.7: Escalation Rules
**Actor:** Manager, System Administrator
**Description:** Administrator configures automatic escalation of unresolved faults after set time.

### UC-26.8: SLA Monitoring
**Actor:** Manager
**Description:** Manager monitors service level agreements and receives alerts for potential breaches.

### UC-26.9: Bulk Operations
**Actor:** Manager
**Description:** Manager performs actions on multiple items simultaneously (bulk update, bulk delete).

### UC-26.10: Scheduled Tasks
**Actor:** Manager, System Administrator
**Description:** Administrator schedules any action for automatic future execution.

---

## UC-27: Multi-tenancy & Organizations (ppt-web, mobile)

### UC-27.1: Create Organization
**Actor:** Super Administrator
**Description:** Super admin creates a new organization (housing cooperative or property management company).

### UC-27.2: Edit Organization
**Actor:** Super Administrator, Organization Admin
**Description:** Administrator edits organization details (name, contact info, settings).

### UC-27.3: Delete Organization
**Actor:** Super Administrator
**Description:** Super admin permanently deletes an organization and all associated data.

### UC-27.4: View Organization List
**Actor:** Super Administrator
**Description:** Super admin views list of all organizations on the platform.

### UC-27.5: Assign Building to Organization
**Actor:** Super Administrator, Organization Admin
**Description:** Administrator assigns a building to be managed by an organization.

### UC-27.6: Remove Building from Organization
**Actor:** Super Administrator, Organization Admin
**Description:** Administrator removes a building from an organization's management.

### UC-27.7: Switch Organization Context
**Actor:** Manager, Organization Admin
**Description:** User with access to multiple organizations switches between them.

### UC-27.8: View Organization Statistics
**Actor:** Organization Admin, Manager
**Description:** Administrator views statistics for their organization (buildings, users, activity).

### UC-27.9: Configure Organization Settings
**Actor:** Organization Admin
**Description:** Administrator configures organization-specific settings and preferences.

### UC-27.10: Manage Organization Branding
**Actor:** Organization Admin
**Description:** Administrator customizes organization branding (logo, colors, email templates).

---

## UC-28: Delegation & Permissions (ppt-web, mobile)

### UC-28.1: Delegate Rights to Person
**Actor:** Owner
**Description:** Owner delegates specific rights to another person (family member, representative).

### UC-28.2: Revoke Delegated Rights
**Actor:** Owner
**Description:** Owner revokes previously delegated rights from a person.

### UC-28.3: View Active Delegations
**Actor:** Owner, Owner Delegate
**Description:** User views list of all active delegations (given or received).

### UC-28.4: Accept Delegation Invitation
**Actor:** Owner Delegate
**Description:** Person accepts an invitation to act as delegate for an owner.

### UC-28.5: Decline Delegation Invitation
**Actor:** Owner Delegate
**Description:** Person declines an invitation to act as delegate for an owner.

### UC-28.6: Set Delegation Expiry Date
**Actor:** Owner
**Description:** Owner sets an expiry date for delegated rights (temporary delegation).

### UC-28.7: Delegate Voting Rights
**Actor:** Owner
**Description:** Owner specifically delegates voting rights for owners' meetings.

### UC-28.8: Delegate Payment Rights
**Actor:** Owner
**Description:** Owner delegates rights to make payments on their behalf.

### UC-28.9: View Delegation History
**Actor:** Owner, Manager
**Description:** User views historical record of all delegations for a unit.

### UC-28.10: Notify on Delegation Expiry
**Actor:** AI System
**Description:** System notifies owner and delegate when delegation is about to expire.

---

## UC-29: Short-term Rental Management (Airbnb/Booking) (ppt-web, mobile)

### UC-29.1: Connect Airbnb Account
**Actor:** Property Manager, Owner
**Description:** User connects their Airbnb account to sync reservations automatically.

### UC-29.2: Connect Booking.com Account
**Actor:** Property Manager, Owner
**Description:** User connects their Booking.com account to sync reservations automatically.

### UC-29.3: Sync Reservations
**Actor:** Property Manager, AI System
**Description:** System synchronizes reservations from connected platforms.

### UC-29.4: View Reservation Calendar
**Actor:** Property Manager, Owner
**Description:** User views calendar with all reservations across platforms.

### UC-29.5: Register Guest from Reservation
**Actor:** Property Manager, AI System
**Description:** System automatically creates guest registration from reservation data.

### UC-29.6: Generate Access Code for Guest
**Actor:** Property Manager, AI System
**Description:** System generates temporary access code for guest's stay duration.

### UC-29.7: Send Welcome Message to Guest
**Actor:** Property Manager, AI System
**Description:** System sends automated welcome message with check-in instructions.

### UC-29.8: Auto-generate Police Registration
**Actor:** AI System
**Description:** System automatically generates police registration form from guest data.

### UC-29.9: Track Guest Check-in
**Actor:** Property Manager, Guest
**Description:** System tracks when guest completes check-in process.

### UC-29.10: Track Guest Check-out
**Actor:** Property Manager, Guest
**Description:** System tracks when guest completes check-out process.

### UC-29.11: Rate Guest
**Actor:** Property Manager
**Description:** Property manager rates guest after their stay for future reference.

### UC-29.12: Block Problem Guest
**Actor:** Property Manager
**Description:** Property manager blocks a problematic guest from future bookings.

### UC-29.13: View Rental Statistics
**Actor:** Property Manager, Owner
**Description:** User views statistics on occupancy, revenue, and guest ratings.

### UC-29.14: Calculate Rental Income
**Actor:** Property Manager, Owner
**Description:** System calculates total rental income for a period.

### UC-29.15: Export Tax Report
**Actor:** Property Manager, Owner
**Description:** User exports rental income report for tax purposes.

---

## UC-30: Guest Registration System (ppt-web, mobile)

### UC-30.1: Register Guest Manually
**Actor:** Property Manager, Owner
**Description:** User manually registers a guest by entering their details.

### UC-30.2: Scan Guest ID Document
**Actor:** Property Manager, Guest
**Description:** User scans guest's ID document using device camera.

### UC-30.3: OCR Extract Guest Data
**Actor:** AI System
**Description:** AI extracts guest information from scanned ID document.

### UC-30.4: Submit to Police Registry
**Actor:** Property Manager, AI System
**Description:** System submits guest registration to police/government registry.

### UC-30.5: View Guest History
**Actor:** Property Manager, Owner, Manager
**Description:** User views history of all guests who stayed at a unit.

### UC-30.6: Search Guests
**Actor:** Property Manager, Manager
**Description:** User searches registered guests by name, date, or nationality.

### UC-30.7: Export Guest List
**Actor:** Property Manager, Manager
**Description:** User exports guest list for reporting or compliance purposes.

### UC-30.8: Generate Guest Statistics
**Actor:** Property Manager, Manager
**Description:** System generates statistics on guest demographics and stays.

### UC-30.9: Set Guest Notification Rules
**Actor:** Property Manager, Manager
**Description:** User configures automatic notifications for guest-related events.

### UC-30.10: Archive Old Guest Records
**Actor:** Manager, AI System
**Description:** System archives old guest records according to retention policy.

---

## UC-31: Real Estate & Listings (ppt-web, mobile, reality-web, mobile-native)

### UC-31.1: Create Property Listing (Sale)
**Actor:** Owner, Real Estate Agent
**Description:** User creates a listing to sell a property.

### UC-31.2: Create Property Listing (Rent)
**Actor:** Owner, Real Estate Agent
**Description:** User creates a listing to rent out a property.

### UC-31.3: Edit Listing
**Actor:** Owner, Real Estate Agent
**Description:** User edits an existing property listing.

### UC-31.4: Publish Listing
**Actor:** Owner, Real Estate Agent
**Description:** User publishes a listing to make it visible to potential buyers/tenants.

### UC-31.5: Unpublish Listing
**Actor:** Owner, Real Estate Agent
**Description:** User temporarily hides a listing from public view.

### UC-31.6: Upload Listing Photos
**Actor:** Owner, Real Estate Agent
**Description:** User uploads photos for a property listing.

### UC-31.7: Generate Virtual Tour
**Actor:** Real Estate Agent, AI System
**Description:** System generates a virtual tour from uploaded photos.

### UC-31.8: Assign Real Estate Agent
**Actor:** Owner
**Description:** Owner assigns a real estate agent to manage their listing.

### UC-31.9: Track Listing Views
**Actor:** Owner, Real Estate Agent
**Description:** User views analytics on listing views and engagement.

### UC-31.10: Manage Inquiries
**Actor:** Owner, Real Estate Agent
**Description:** User manages and responds to inquiries about a listing.

### UC-31.11: Schedule Viewing
**Actor:** Real Estate Agent, Owner
**Description:** User schedules a property viewing with interested party.

### UC-31.12: Record Viewing Feedback
**Actor:** Real Estate Agent
**Description:** Agent records feedback from property viewing.

### UC-31.13: Mark as Sold/Rented
**Actor:** Owner, Real Estate Agent
**Description:** User marks a property as sold or rented.

### UC-31.14: Archive Listing
**Actor:** Owner, Real Estate Agent
**Description:** User archives an old listing for historical reference.

---

## UC-32: Real Estate Portal Integration (API) (ppt-web, mobile, reality-web, mobile-native)

### UC-32.1: Configure Portal Connection
**Actor:** Organization Admin, Real Estate Agent
**Description:** User configures connection to external real estate portal.

### UC-32.2: Export Listing to Portal
**Actor:** Real Estate Agent, AI System
**Description:** System exports a listing to connected real estate portal.

### UC-32.3: Sync Listing Updates
**Actor:** AI System
**Description:** System synchronizes listing updates with connected portals.

### UC-32.4: Remove Listing from Portal
**Actor:** Real Estate Agent
**Description:** User removes a listing from external portal.

### UC-32.5: Import Inquiries from Portal
**Actor:** AI System
**Description:** System imports inquiries received through external portals.

### UC-32.6: View Portal Statistics
**Actor:** Real Estate Agent, Owner
**Description:** User views performance statistics from each connected portal.

### UC-32.7: Manage Multiple Portals
**Actor:** Real Estate Agent
**Description:** User manages connections to multiple real estate portals.

### UC-32.8: Auto-refresh Listings
**Actor:** AI System
**Description:** System automatically refreshes listings on portals to maintain visibility.

### UC-32.9: Handle Portal Webhooks
**Actor:** Real Estate Portal, AI System
**Description:** System processes incoming webhooks from real estate portals.

### UC-32.10: Generate Portal Report
**Actor:** Real Estate Agent, Manager
**Description:** User generates report on portal performance and conversions.

---

## UC-33: Tenant Screening (ppt-web, mobile)

### UC-33.1: Request Tenant Background Check
**Actor:** Owner, Real Estate Agent
**Description:** User requests a background check for a potential tenant.

### UC-33.2: Verify Tenant Income
**Actor:** Real Estate Agent, Owner
**Description:** User verifies tenant's income through documentation or third-party service.

### UC-33.3: Check Tenant References
**Actor:** Real Estate Agent
**Description:** Agent contacts and verifies tenant's references.

### UC-33.4: View Tenant Credit Score
**Actor:** Owner, Real Estate Agent
**Description:** User views tenant's credit score from credit bureau.

### UC-33.5: Request Employer Verification
**Actor:** Real Estate Agent
**Description:** Agent requests verification of tenant's employment.

### UC-33.6: Generate Tenant Report
**Actor:** Real Estate Agent, AI System
**Description:** System generates comprehensive tenant screening report.

### UC-33.7: Approve Tenant Application
**Actor:** Owner, Real Estate Agent
**Description:** User approves a tenant application after screening.

### UC-33.8: Reject Tenant Application
**Actor:** Owner, Real Estate Agent
**Description:** User rejects a tenant application with documented reason.

### UC-33.9: Store Screening Results
**Actor:** AI System
**Description:** System securely stores tenant screening results.

### UC-33.10: Compare Tenant Applications
**Actor:** Owner, Real Estate Agent
**Description:** User compares multiple tenant applications side by side.

### UC-33.11: GDPR-Compliant Screening
**Actor:** Real Estate Agent, Owner
**Description:** User performs tenant screening using GDPR-compliant processes with explicit consent in EU regions.

### UC-33.12: Manage Screening Consent
**Actor:** Tenant
**Description:** Prospective tenant reviews and provides consent for credit check and background screening.

---

## UC-34: Lease Management (ppt-web, mobile)

### UC-34.1: Create Lease Agreement
**Actor:** Owner, Real Estate Agent, Manager
**Description:** User creates a new lease agreement for a tenant.

### UC-34.2: Generate Lease from Template
**Actor:** Owner, Real Estate Agent, Manager
**Description:** User generates lease agreement from predefined template.

### UC-34.3: Send Lease for Signature
**Actor:** Owner, Real Estate Agent, Manager
**Description:** User sends lease agreement for electronic signature.

### UC-34.4: Track Lease Signature Status
**Actor:** Owner, Real Estate Agent
**Description:** User tracks the signature status of sent lease agreements.

### UC-34.5: Store Signed Lease
**Actor:** AI System
**Description:** System securely stores fully executed lease agreement.

### UC-34.6: Set Lease Renewal Reminder
**Actor:** Owner, Manager, AI System
**Description:** System sets reminder before lease expiration for renewal decision.

### UC-34.7: Renew Lease
**Actor:** Owner, Tenant
**Description:** User initiates lease renewal process.

### UC-34.8: Terminate Lease
**Actor:** Owner, Tenant, Manager
**Description:** User initiates early lease termination process.

### UC-34.9: Calculate Lease Balance
**Actor:** Manager, AI System
**Description:** System calculates outstanding balance at lease end.

### UC-34.10: Track Lease Violations
**Actor:** Manager, Owner
**Description:** User documents and tracks lease violations by tenant.

---

## UC-35: Insurance Management (ppt-web, mobile)

### UC-35.1: View Building Insurance Policies
**Actor:** Manager, Organization Admin
**Description:** User views all active insurance policies for a building.

### UC-35.2: Add Insurance Policy
**Actor:** Manager, Organization Admin
**Description:** Administrator adds a new insurance policy with coverage details and expiry date.

### UC-35.3: File Insurance Claim
**Actor:** Manager
**Description:** Manager files an insurance claim for damage or incident.

### UC-35.4: Track Claim Status
**Actor:** Manager, Organization Admin
**Description:** User tracks the status of submitted insurance claims.

### UC-35.5: Upload Claim Documentation
**Actor:** Manager
**Description:** Manager uploads supporting documents (photos, reports) for an insurance claim.

### UC-35.6: View Claim History
**Actor:** Manager, Organization Admin
**Description:** User views historical insurance claims for a building.

### UC-35.7: Set Policy Renewal Reminder
**Actor:** Manager, AI System
**Description:** System sends reminder before insurance policy expiration.

### UC-35.8: Compare Insurance Quotes
**Actor:** Manager, Organization Admin
**Description:** User compares quotes from different insurance providers.

---

## UC-36: Maintenance Scheduling (ppt-web, mobile)

### UC-36.1: Schedule Preventive Maintenance
**Actor:** Manager, Technical Manager
**Description:** User schedules preventive maintenance for building equipment.

### UC-36.2: View Maintenance Calendar
**Actor:** Manager, Technical Manager
**Description:** User views calendar of all scheduled maintenance activities.

### UC-36.3: Assign Maintenance Task
**Actor:** Manager
**Description:** Manager assigns a maintenance task to a technical manager or contractor.

### UC-36.4: Track Maintenance Completion
**Actor:** Manager, Technical Manager
**Description:** User marks maintenance task as complete and adds notes.

### UC-36.5: Set Recurring Maintenance
**Actor:** Manager
**Description:** Manager sets up recurring maintenance schedule (weekly, monthly, annual).

### UC-36.6: Generate Maintenance Report
**Actor:** Manager
**Description:** Manager generates report on maintenance activities and costs.

### UC-36.7: Manage Maintenance Contractors
**Actor:** Manager, Organization Admin
**Description:** Administrator manages list of approved maintenance contractors.

### UC-36.8: View Equipment Maintenance History
**Actor:** Manager, Technical Manager
**Description:** User views maintenance history for specific equipment.

---

## UC-37: Supplier/Vendor Management (ppt-web, mobile)

### UC-37.1: Add Supplier
**Actor:** Manager, Organization Admin
**Description:** Administrator adds a new supplier/vendor to the system.

### UC-37.2: View Supplier Directory
**Actor:** Manager
**Description:** User views directory of all registered suppliers.

### UC-37.3: Rate Supplier Performance
**Actor:** Manager, Technical Manager
**Description:** User rates a supplier after completed work.

### UC-37.4: Track Supplier Invoices
**Actor:** Manager
**Description:** Manager tracks invoices received from suppliers.

### UC-37.5: Manage Supplier Contracts
**Actor:** Manager, Organization Admin
**Description:** Administrator manages contracts with suppliers.

### UC-37.6: Request Quote from Supplier
**Actor:** Manager
**Description:** Manager sends request for quote to one or more suppliers.

### UC-37.7: Compare Supplier Offers
**Actor:** Manager
**Description:** Manager compares quotes received from multiple suppliers.

### UC-37.8: View Supplier Payment History
**Actor:** Manager
**Description:** Manager views payment history for a specific supplier.

---

## UC-38: Legal & Compliance (ppt-web, mobile)

### UC-38.1: Store Legal Documents
**Actor:** Manager, Organization Admin
**Description:** Administrator uploads and stores legal documents securely.

### UC-38.2: Track Regulatory Deadlines
**Actor:** Manager, AI System
**Description:** System tracks and reminds about regulatory compliance deadlines.

### UC-38.3: Generate Compliance Report
**Actor:** Manager, Organization Admin
**Description:** User generates report on regulatory compliance status.

### UC-38.4: Manage Building Permits
**Actor:** Manager, Organization Admin
**Description:** Administrator manages building permits and their validity.

### UC-38.5: Track Safety Inspections
**Actor:** Manager, Technical Manager
**Description:** User tracks mandatory safety inspections (fire, elevator, electrical).

### UC-38.6: Document Legal Disputes
**Actor:** Manager, Organization Admin
**Description:** Administrator documents ongoing legal disputes with details and status.

### UC-38.7: Archive Court Documents
**Actor:** Manager, Organization Admin
**Description:** Administrator archives court documents and legal correspondence.

### UC-38.8: View Compliance Calendar
**Actor:** Manager
**Description:** Manager views calendar of all compliance deadlines and inspections.

---

## UC-39: Emergency Management (ppt-web, mobile)

### UC-39.1: Trigger Emergency Alert
**Actor:** Manager, System Administrator
**Description:** Administrator sends emergency alert to all building residents.

### UC-39.2: View Evacuation Plan
**Actor:** Owner, Tenant, Resident, Guest
**Description:** User views building evacuation plan and routes.

### UC-39.3: Track Emergency Response
**Actor:** Manager
**Description:** Manager tracks response to an active emergency situation.

### UC-39.4: Manage Emergency Contacts
**Actor:** Manager
**Description:** Manager maintains list of emergency contacts (fire, police, hospital).

### UC-39.5: Conduct Emergency Drill
**Actor:** Manager
**Description:** Manager schedules and documents emergency evacuation drills.

### UC-39.6: Report Safety Hazard
**Actor:** Owner, Tenant, Resident
**Description:** User reports a safety hazard in the building.

### UC-39.7: View Emergency Procedures
**Actor:** Owner, Tenant, Resident, Guest
**Description:** User views documented emergency procedures (fire, flood, gas leak).

### UC-39.8: Send Emergency Broadcast
**Actor:** Manager, AI System
**Description:** System sends mass notification via multiple channels during emergency.

---

## UC-40: Budget & Planning (ppt-web, mobile)

### UC-40.1: Create Annual Budget
**Actor:** Manager, Organization Admin
**Description:** Administrator creates annual budget for a building.

### UC-40.2: Track Budget vs Actual
**Actor:** Manager, Organization Admin
**Description:** User compares actual expenses against budgeted amounts.

### UC-40.3: Plan Capital Expenditures
**Actor:** Manager, Organization Admin
**Description:** Administrator plans major capital expenditures (renovations, equipment).

### UC-40.4: Vote on Budget Approval
**Actor:** Owner
**Description:** Owner votes to approve or reject proposed annual budget.

### UC-40.5: Generate Budget Report
**Actor:** Manager
**Description:** Manager generates detailed budget report for stakeholders.

### UC-40.6: Forecast Expenses
**Actor:** Manager, AI System
**Description:** AI forecasts future expenses based on historical data.

### UC-40.7: Manage Reserve Fund
**Actor:** Manager, Organization Admin
**Description:** Administrator manages building reserve fund for major repairs.

### UC-40.8: View Budget History
**Actor:** Owner, Manager
**Description:** User views historical budgets and their outcomes.

---

## UC-41: Subscription & Billing (Platform) (ppt-web)

### UC-41.1: View Subscription Plan
**Actor:** Organization Admin
**Description:** Administrator views current subscription plan and features.

### UC-41.2: Upgrade Subscription
**Actor:** Organization Admin
**Description:** Administrator upgrades to a higher subscription tier.

### UC-41.3: Downgrade Subscription
**Actor:** Organization Admin
**Description:** Administrator downgrades to a lower subscription tier.

### UC-41.4: View Platform Billing History
**Actor:** Organization Admin
**Description:** Administrator views billing history for platform subscription.

### UC-41.5: Update Payment Method
**Actor:** Organization Admin
**Description:** Administrator updates payment method for subscription.

### UC-41.6: Download Platform Invoice
**Actor:** Organization Admin
**Description:** Administrator downloads invoice for platform subscription.

### UC-41.7: Cancel Subscription
**Actor:** Organization Admin
**Description:** Administrator cancels platform subscription.

### UC-41.8: Apply Discount Code
**Actor:** Organization Admin
**Description:** Administrator applies promotional discount code.

### UC-41.9: Start Free Trial
**Actor:** Organization Admin
**Description:** New organization starts a free trial period with limited features or time.

### UC-41.10: Handle Trial Expiration
**Actor:** AI System, Organization Admin
**Description:** System notifies admin before trial expires and handles transition to paid or limited plan.

### UC-41.11: Calculate Usage-Based Billing
**Actor:** AI System
**Description:** System calculates billing based on usage metrics (buildings, users, API calls).

---

## UC-42: Onboarding & Help (ppt-web, mobile, reality-web, mobile-native)

### UC-42.1: Complete Onboarding Tour
**Actor:** Owner, Tenant, Manager
**Description:** New user completes interactive onboarding tour of the application.

### UC-42.2: View Contextual Help
**Actor:** Owner, Tenant, Manager
**Description:** User views context-sensitive help for current screen.

### UC-42.3: Watch Video Tutorial
**Actor:** Owner, Tenant, Manager
**Description:** User watches video tutorial for a feature.

### UC-42.4: Search FAQ
**Actor:** Owner, Tenant, Manager
**Description:** User searches frequently asked questions database.

### UC-42.5: View Feature Announcements
**Actor:** Owner, Tenant, Manager
**Description:** User views announcements about new features and updates.

### UC-42.6: Submit Feedback
**Actor:** Owner, Tenant, Manager
**Description:** User submits feedback or feature request.

### UC-42.7: Start Support Chat
**Actor:** Owner, Tenant, Manager
**Description:** User initiates live chat with support team.

### UC-42.8: Report Bug
**Actor:** Owner, Tenant, Manager
**Description:** User reports a bug or technical issue.

---

## UC-43: Mobile App Features (mobile, mobile-native)

### UC-43.1: Add Home Screen Widget
**Actor:** Owner, Tenant, Manager
**Description:** User adds dashboard widget to device home screen.

### UC-43.2: Configure Quick Actions
**Actor:** Owner, Tenant, Manager
**Description:** User configures quick actions accessible from lock screen.

### UC-43.3: Use Voice Assistant
**Actor:** Owner, Tenant
**Description:** User controls app features via Siri/Google Assistant.

### UC-43.4: Scan QR Code
**Actor:** Owner, Tenant, Manager, Guest
**Description:** User scans QR code for quick access to features or information.

### UC-43.5: Use NFC for Access
**Actor:** Owner, Tenant
**Description:** User uses NFC-enabled device for building access.

### UC-43.6: Receive Critical Push Alerts
**Actor:** Owner, Tenant, Resident
**Description:** User receives high-priority push notifications for emergencies.

### UC-43.7: Use Dark Mode
**Actor:** Owner, Tenant, Manager
**Description:** User enables dark mode for reduced eye strain.

### UC-43.8: Configure App Badge
**Actor:** Owner, Tenant, Manager
**Description:** User configures app badge to show unread count.

---

## UC-44: Favorites Management (reality-web, mobile-native)

### UC-44.1: Add Listing to Favorites
**Actor:** Portal User
**Description:** User saves a listing to their favorites list for later viewing.

### UC-44.2: Remove from Favorites
**Actor:** Portal User
**Description:** User removes a listing from their favorites list.

### UC-44.3: View Favorites List
**Actor:** Portal User
**Description:** User views all saved favorite listings in one place.

### UC-44.4: Share Favorites
**Actor:** Portal User
**Description:** User shares their favorites list via link or email with others.

### UC-44.5: Export Favorites
**Actor:** Portal User
**Description:** User exports favorites list to PDF or spreadsheet format.

### UC-44.6: Favorites Notifications
**Actor:** Portal User, AI System
**Description:** User receives alerts when a favorite listing's price changes or status updates.

---

## UC-45: Saved Searches & Alerts (reality-web, mobile-native)

### UC-45.1: Save Search Criteria
**Actor:** Portal User
**Description:** User saves current search filters and criteria for future use.

### UC-45.2: View Saved Searches
**Actor:** Portal User
**Description:** User views list of all saved search configurations.

### UC-45.3: Edit Saved Search
**Actor:** Portal User
**Description:** User modifies the criteria of a saved search.

### UC-45.4: Delete Saved Search
**Actor:** Portal User
**Description:** User removes a saved search from their list.

### UC-45.5: Enable Search Alerts
**Actor:** Portal User
**Description:** User enables notifications for new listings matching saved search criteria.

### UC-45.6: Configure Alert Frequency
**Actor:** Portal User
**Description:** User sets how often to receive alerts (instant, daily, weekly).

### UC-45.7: Receive New Listing Alert
**Actor:** Portal User, AI System
**Description:** System sends notification when a new listing matches saved search criteria.

### UC-45.8: Receive Price Drop Alert
**Actor:** Portal User, AI System
**Description:** System notifies user when a listing's price decreases.

---

## UC-46: Contact Inquiries (reality-web, mobile-native)

### UC-46.1: Send Contact Message
**Actor:** Portal User
**Description:** User sends an inquiry message about a listing to the owner or agent.

### UC-46.2: Request Viewing
**Actor:** Portal User
**Description:** User requests to schedule a property viewing appointment.

### UC-46.3: View Inquiry History
**Actor:** Portal User
**Description:** User views their sent inquiries and any responses received.

### UC-46.4: Track Inquiry Status
**Actor:** Portal User
**Description:** User sees status of inquiries (pending, responded, scheduled).

### UC-46.5: Respond to Inquiry
**Actor:** Owner, Real Estate Agent, Realtor
**Description:** Listing owner or agent responds to a user inquiry.

### UC-46.6: Schedule Viewing from Inquiry
**Actor:** Real Estate Agent, Realtor
**Description:** Agent schedules a property viewing based on user's request.

---

## UC-47: Portal User Accounts (reality-web, mobile-native)

### UC-47.1: Register Portal Account
**Actor:** Portal User
**Description:** User creates a new account on Reality Portal with email and password.

### UC-47.2: Login with Email/Password
**Actor:** Portal User, Realtor, Agency Manager, Agency Owner
**Description:** User logs into the portal using their email and password credentials.

### UC-47.3: Login with Google
**Actor:** Portal User, Realtor, Agency Manager, Agency Owner
**Description:** User authenticates using their Google account via OAuth.

### UC-47.4: Login with Apple
**Actor:** Portal User, Realtor, Agency Manager, Agency Owner
**Description:** User authenticates using their Apple ID via Sign in with Apple.

### UC-47.5: Login with Facebook
**Actor:** Portal User, Realtor, Agency Manager, Agency Owner
**Description:** User authenticates using their Facebook account via OAuth.

### UC-47.6: Link Property Management Account
**Actor:** Portal User
**Description:** User links their portal account to their Property Management account for SSO.

### UC-47.7: View Portal Profile
**Actor:** Portal User, Realtor
**Description:** User views their profile information and account settings.

### UC-47.8: Update Portal Profile
**Actor:** Portal User, Realtor
**Description:** User edits their profile information (name, avatar, preferences).

### UC-47.9: Delete Portal Account
**Actor:** Portal User
**Description:** User requests deletion of their account and personal data (GDPR).

### UC-47.10: Manage Linked Social Accounts
**Actor:** Portal User, Realtor
**Description:** User links or unlinks Google, Apple, or Facebook accounts.

### UC-47.11: Register as Realtor
**Actor:** Realtor
**Description:** Agent registers as a realtor with license verification information.

### UC-47.12: Create Reality Agency
**Actor:** Agency Owner
**Description:** Owner creates a new reality agency organization on the portal.

### UC-47.13: Invite Realtors to Agency
**Actor:** Agency Owner, Agency Manager
**Description:** Manager sends invitations to realtors to join the agency.

### UC-47.14: Accept Agency Invitation
**Actor:** Realtor
**Description:** Realtor accepts an invitation to join a reality agency.

### UC-47.15: Manage Agency Realtors
**Actor:** Agency Owner, Agency Manager
**Description:** Manager views and manages the list of agency's realtors.

---

## UC-48: Property Comparison (reality-web, mobile-native)

### UC-48.1: Add to Comparison
**Actor:** Portal User
**Description:** User adds a listing to their comparison list.

### UC-48.2: View Comparison
**Actor:** Portal User
**Description:** User views side-by-side comparison of selected properties.

### UC-48.3: Remove from Comparison
**Actor:** Portal User
**Description:** User removes a listing from the comparison list.

### UC-48.4: Share Comparison
**Actor:** Portal User
**Description:** User shares their property comparison via link.

### UC-48.5: Export Comparison
**Actor:** Portal User
**Description:** User exports property comparison to PDF format.

---

## UC-49: Agency Management (reality-web, mobile-native)

### UC-49.1: View Agency Dashboard
**Actor:** Agency Owner, Agency Manager
**Description:** User views agency performance metrics and overview dashboard.

### UC-49.2: Edit Agency Profile
**Actor:** Agency Owner
**Description:** Owner updates agency name, logo, contact info, and description.

### UC-49.3: Set Agency Branding
**Actor:** Agency Owner
**Description:** Owner configures agency colors and logo watermarks for listings.

### UC-49.4: View Agency Listings
**Actor:** Agency Owner, Agency Manager
**Description:** User views all listings from all agency realtors.

### UC-49.5: View Agency Inquiries
**Actor:** Agency Owner, Agency Manager
**Description:** User views all inquiries received by the agency.

### UC-49.6: Assign Inquiry to Realtor
**Actor:** Agency Manager
**Description:** Manager assigns an incoming inquiry to a specific realtor.

### UC-49.7: View Realtor Performance
**Actor:** Agency Owner, Agency Manager
**Description:** User views realtor metrics (listings, inquiries, conversions).

### UC-49.8: Suspend Realtor
**Actor:** Agency Owner
**Description:** Owner temporarily disables a realtor's access to the agency.

### UC-49.9: Remove Realtor from Agency
**Actor:** Agency Owner
**Description:** Owner removes a realtor from the agency.

### UC-49.10: Configure Agency Settings
**Actor:** Agency Owner
**Description:** Owner sets agency-wide preferences and policies.

---

## UC-50: Property Import (reality-web, mobile-native)

### UC-50.1: Connect External CRM
**Actor:** Realtor, Agency Owner
**Description:** User connects to an external CRM system using API credentials.

### UC-50.2: Import Properties from CRM
**Actor:** Realtor, Agency Owner
**Description:** User imports property listings from a connected CRM system.

### UC-50.3: Map CRM Fields
**Actor:** Realtor, Agency Owner
**Description:** User configures field mapping between CRM and portal fields.

### UC-50.4: Schedule Automatic Sync
**Actor:** Agency Owner
**Description:** Owner sets up periodic automatic import from CRM.

### UC-50.5: View Import History
**Actor:** Realtor, Agency Owner
**Description:** User views log of past imports and their status.

### UC-50.6: Resolve Import Conflicts
**Actor:** Realtor
**Description:** User handles duplicate or conflicting listings during import.

### UC-50.7: Import from XML Feed
**Actor:** Agency Owner
**Description:** Owner imports properties from an XML/RSS feed URL.

### UC-50.8: Import from CSV File
**Actor:** Realtor
**Description:** User bulk imports listings from a CSV or Excel file upload.

### UC-50.9: Import from IDX/RETS
**Actor:** Agency Owner
**Description:** Owner connects to MLS/IDX real estate data feeds for import.

### UC-50.10: Export Properties
**Actor:** Realtor
**Description:** User exports their listings to external formats.

---

## UC-51: Realtor Profile & Listings (reality-web, mobile-native)

### UC-51.1: Create Realtor Profile
**Actor:** Realtor
**Description:** Realtor sets up professional profile with bio, photo, and credentials.

### UC-51.2: Add License Information
**Actor:** Realtor
**Description:** Realtor adds real estate license details for verification.

### UC-51.3: View My Listings
**Actor:** Realtor
**Description:** Realtor views all listings they have created.

### UC-51.4: Create New Listing
**Actor:** Realtor
**Description:** Realtor creates a new property listing with details.

### UC-51.5: Edit Listing
**Actor:** Realtor
**Description:** Realtor modifies an existing listing's details.

### UC-51.6: Upload Listing Photos
**Actor:** Realtor
**Description:** Realtor adds and manages photos for a listing.

### UC-51.7: Set Listing Status
**Actor:** Realtor
**Description:** Realtor changes listing status (active, pending, sold, withdrawn).

### UC-51.8: View My Inquiries
**Actor:** Realtor
**Description:** Realtor views inquiries received for their listings.

### UC-51.9: Respond to Inquiry
**Actor:** Realtor
**Description:** Realtor replies to a potential buyer or renter inquiry.

### UC-51.10: View Listing Analytics
**Actor:** Realtor
**Description:** Realtor views views, favorites, and inquiries per listing.

### UC-51.11: Feature Listing
**Actor:** Realtor
**Description:** Realtor promotes a listing to featured/premium placement.

### UC-51.12: Share Listing
**Actor:** Realtor
**Description:** Realtor shares a listing on social media or via link.

---

## Summary

| Category | Use Cases | Apps |
|----------|-----------|------|
| UC-01: Notifications | 6 | ppt-web, mobile |
| UC-02: Announcements | 13 | ppt-web, mobile |
| UC-03: Faults | 14 | ppt-web, mobile |
| UC-04: Voting and Polls | 14 | ppt-web, mobile |
| UC-05: Messages | 11 | ppt-web, mobile |
| UC-06: Neighbors | 10 | ppt-web, mobile |
| UC-07: Contacts | 7 | ppt-web, mobile |
| UC-08: Documents | 14 | ppt-web, mobile |
| UC-09: Forms | 8 | ppt-web, mobile |
| UC-10: Person-Months | 7 | ppt-web, mobile |
| UC-11: Self-Readings | 10 | ppt-web, mobile |
| UC-12: Outages | 8 | ppt-web, mobile |
| UC-13: News | 9 | ppt-web, mobile |
| UC-14: User Account Management | 12 | ppt-web, mobile, reality-web, mobile-native |
| UC-15: Building/Property Management | 10 | ppt-web, mobile |
| UC-16: Financial Management | 10 | ppt-web, mobile |
| UC-17: Reports and Analytics | 5 | ppt-web |
| UC-18: System Administration | 6 | ppt-web |
| UC-19: Real-time & Mobile Features | 12 | ppt-web, mobile, reality-web, mobile-native |
| UC-20: AI/ML Features | 24 | ppt-web, mobile, reality-web, mobile-native |
| UC-21: IoT & Smart Building | 10 | ppt-web, mobile |
| UC-22: External Integrations | 10 | ppt-web, mobile |
| UC-23: Security & Compliance | 12 | ppt-web, mobile, reality-web, mobile-native |
| UC-24: Community & Social | 10 | ppt-web, mobile |
| UC-25: Accessibility | 8 | ppt-web, mobile, reality-web, mobile-native |
| UC-26: Workflow Automation | 10 | ppt-web, mobile |
| UC-27: Multi-tenancy & Organizations | 10 | ppt-web, mobile |
| UC-28: Delegation & Permissions | 10 | ppt-web, mobile |
| UC-29: Short-term Rental Management | 15 | ppt-web, mobile |
| UC-30: Guest Registration System | 10 | ppt-web, mobile |
| UC-31: Real Estate & Listings | 14 | ppt-web, mobile, reality-web, mobile-native |
| UC-32: Real Estate Portal Integration | 10 | ppt-web, mobile, reality-web, mobile-native |
| UC-33: Tenant Screening | 12 | ppt-web, mobile |
| UC-34: Lease Management | 10 | ppt-web, mobile |
| UC-35: Insurance Management | 8 | ppt-web, mobile |
| UC-36: Maintenance Scheduling | 8 | ppt-web, mobile |
| UC-37: Supplier/Vendor Management | 8 | ppt-web, mobile |
| UC-38: Legal & Compliance | 8 | ppt-web, mobile |
| UC-39: Emergency Management | 8 | ppt-web, mobile |
| UC-40: Budget & Planning | 8 | ppt-web, mobile |
| UC-41: Subscription & Billing | 11 | ppt-web |
| UC-42: Onboarding & Help | 8 | ppt-web, mobile, reality-web, mobile-native |
| UC-43: Mobile App Features | 8 | mobile, mobile-native |
| UC-44: Favorites Management | 6 | reality-web, mobile-native |
| UC-45: Saved Searches & Alerts | 8 | reality-web, mobile-native |
| UC-46: Contact Inquiries | 6 | reality-web, mobile-native |
| UC-47: Portal User Accounts | 15 | reality-web, mobile-native |
| UC-48: Property Comparison | 5 | reality-web, mobile-native |
| UC-49: Agency Management | 10 | reality-web, mobile-native |
| UC-50: Property Import | 10 | reality-web, mobile-native |
| UC-51: Realtor Profile & Listings | 12 | reality-web, mobile-native |
| **TOTAL** | **508** | |
