## Development Status
Finished:
- Send ONE file;
- Receive ONE file;
- Modify settings;
- Open received files or open its directory;
- Auto start;
- Dark/Light mode;
- Send texts;
- Send multiple files(zip/unzip);
---
  Issues:
- Auto start not working;
---
  To-do:
- Refactor code;
- Drag and drop support;
---
Future:
- Support Android / iOS;
- Keyboard shortcuts;



--- 
sequence diagram for sending/receiving files:

```mermaid
sequenceDiagram
   participant User
   participant MainPanel
   participant SendDialog
   participant Backend

   User->>MainPanel: Select file(s)
   MainPanel-->>User: Display selected file(s)
   User->>MainPanel: Click "Send" button
   MainPanel->>SendDialog: Open Send Dialog with files
   alt More than one file
      SendDialog->>Backend: `zip_files`
      Backend-->>SendDialog: Return zipped file
   end
   SendDialog->>Backend: `upload_file`
   Backend-->>SendDialog: Emit `upload_started`
   alt Upload successful
      Backend-->>SendDialog: Emit `upload_success`
   else Upload failed
      Backend-->>SendDialog: Emit `upload_failed`
   end
```

```mermaid
sequenceDiagram
    participant User
    participant MainPanel
    participant ReceiveDialog
    participant Backend
    
    User->>MainPanel: Click "Peek" function
    MainPanel->>Backend: `peek_file`
    Backend-->>MainPanel: Return the latest file(s) / error
    MainPanel-->>User: Display file(s)
    User->>ReceiveDialog: Click "Receive" button
    MainPanel->>ReceiveDialog: Open Receive Dialog
    ReceiveDialog->>Backend: `download_file`
    Backend-->>ReceiveDialog: Emit `download_started`
    alt Download successful
        Backend-->>ReceiveDialog: Emit `download_success`
        alt File is zipped by sender
            ReceiveDialog->>Backend: `unzip_files`
            Backend-->>ReceiveDialog: Return files to be displayed
        end
        Backend-->>ReceiveDialog: Display files
    else Download failed
        Backend-->>ReceiveDialog: Emit `download_failed`
    end
```