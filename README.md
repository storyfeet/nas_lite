Nas Lite
=======

A program for managing a simple file server for cross platform use.

Requirements
-----------

To be able to maintain a number of files both shared and personal files.

As a User I can set the system to maintain a folder.
As a user I can tell the system to share folder with other users.
As a user I can maintain a single file, to keep the system working.


Backend
------

Files stored in Database have owner, path, data and hash.  Hash can be used to confirm matches.

Folders also maintain a hash, if a child hash changes the parent will also need updating.


Tables
--------

Folder:
    Name
    Parent
    Path
    Hash 
    Owner
    size

File:
    Name
    Path
    Folder
    Data
    Hash - calculated from direct children
    Owner
    size - calculated from direct children
    expired? - 

User:
    Name


Share:
    Folder:
    Owner:
    Accessor?:
    Token?:
    Permission:


Token:
    Id
    Key
    User?
    Expiry

    
    
