# Property Management System Specification v1.0

Each section is designed with numerous elements that improve communication between property managers and apartment owners. Below is a detailed overview of the features of individual modules and their user interface:

## Dashboard and Announcements

* **Notifications** – at the top of the dashboard, an information banner with an "Enable notifications" button is displayed, allowing users to activate push notifications for new messages, events, or announcements.
* **Add Announcement** – in each announcements section there is an "Add" button that opens a form for creating a new announcement (entering text, optional attachments, and visibility settings). The same workflow is used for creating a regular announcement; for the special "Owners' Meeting" announcement, the exact date and time of the meeting is additionally specified.
* **Search and Filters** – the "Search" field enables full-text search of announcements by title or text. The following filters are also available on the announcements page:

  * **Status** – a dropdown menu for selecting the announcement status (e.g., published/draft/archived).
  * **Created by me** – a toggle switch that, when enabled, displays only announcements created by the current user.
* **Announcement Display** – announcements are displayed as cards; each contains:

  * **Announcement Type** (e.g., "Announcement", "Vote", or "Owners' Meeting").
  * **Title and Introduction** – the headline and a few lines of text are displayed; for longer posts, the text is truncated and the detail must be opened.
  * **Attachments** – files attached to the announcement are displayed below the title (e.g., PDF attachments).
  * **Summary Data** – on the right side, the number of comments and publication date are displayed, with the author's name and company below (e.g., "Vavro Martin (H-Probyt)").
  * **Actions** – option to open announcement detail (by clicking on the card), add a comment, or view existing comments.

## Faults

* **Report Fault** – a button at the top of the page opens a form where an owner or manager reports a problem (entering title, description, location, and optionally attaching a photo).
* **Status and Owner Filter** – similar to announcements, a search field and "Status" dropdown filter (e.g., "published", "being resolved", "closed") and a "Reported by me" toggle are available.
* **Resolution Status** – another "Status" filter allows selecting only faults in a certain state, e.g., "all", "new", "in progress", or "closed".
* **Reports Overview** – each fault is displayed in a list with basic description, report date, status, and reporter's name; the detail contains resolution history and communication with the manager.

## Voting and Polls

* **Search and Status** – the interface contains a "Search" field and "Status" filter to display published, pending, or completed votes.
* **Votes** – each item shows the title and brief description of the voting subject, information about the end date, and the number of comments/reactions.
* **Vote Detail** – when opened, you can read the detailed wording of the question, view results, track the number of voters, and add a comment. For ongoing votes, a countdown to the end and a button to cast a vote are displayed; for completed votes, a summary of results is available.

## Messages

* **New Message** – the "New Message" button creates a conversation with a selected manager or neighbor. When writing a message, the recipient is selected from a list of residents or managers.
* **Conversation Search** – the "Search by name" field allows filtering existing conversations by person's name.
* **Conversation List** – each item displays the recipient's initials, full name, date of last message, and text preview. Clicking opens the conversation detail with a chronological transcript of messages.

## Residential Building

This section provides a complete set of tools related to building administration:

### Neighbors

* Overview of registered owners and tenants.
* The "Invite Neighbors" feature allows sending an invitation to register in the application.
* Search by name and "Entrance" filter make it easier to navigate the list.
* Each neighbor record contains name, floor or entrance, and contact (e.g., icon to send a message).

### Contacts

* Directory of managers and technical managers.
* Displays manager's name, their role (e.g., "Technical Manager"), email contact, and link to personal profile.
* Serves for quick contact with the manager when resolving issues.

### Documents

* Archive documents module.
* Search field allows searching for files or folders by name.
* The main folder is used for storing contracts, invoices, and other documents; the manager can create personal folders for individual owners or folders for specific premises.
* In the detail view, a document can be downloaded or forwarded, and the system maintains version history (if available).

### Forms

* Section for managing forms (e.g., applications, requests).
* Allows searching by name and the manager can publish forms for residents to download or print.
* If no forms are added, the page clearly indicates this.

### Person-Months and Self-Readings

Module for managing fees and meter readings:

* **Person-Months** – used for recording the number of persons living in residential units throughout the months, which is the basis for allocating certain costs (e.g., heating, water). An "Add" button is available to enter a new record.
* **Self-Readings** – owners can enter the current meter reading directly into the application, attach a photo of the meter, and send it to managers for verification. The manager then sees an overview of self-readings in a table with export capability.

## Outages

* The "Outages" page is a clear list of current and planned water and electricity supply outages from various suppliers.
* **Division by Commodity** – separate sections for water and electricity, each listing regional suppliers.
* **Contact Information** – each supplier shows a phone number with an active link (tel://), enabling immediate calling from a smartphone.
* **Link to Outage Pages** – the "Show Outages" button redirects to the supplier's official page with detailed outage information.

## News

* This section serves as the property management platform's blog.
* **Search** – in the top corner there is a search field for filtering news by keywords.
* **News Cards** – each item contains a headline (e.g., "Autumn Update"), brief description of new features, and publication date. Some news items have an attached preview image field with attachment count.
* **Reactions** – a heart icon shows the number of reactions the post has received, allowing tracking of individual article popularity.

---

With this detailed mapping of features, you can create a complete picture of how a property management platform works. This will allow you to plan functionality when developing your own solution.
