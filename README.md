### server for controlling metal hardware
This is meant to be used as the backend server component of a server-client model for managing metal. Will need a separate gui/cli component.
Intended to use mariadb for storage, JWT for auth tokens (but stored in db so they can be revoked). enum for login function should make creating oauth providers easy.
### what it's meant to do:
  - let you make virtual machines
  - let you make containers
  - let you manage storage locally
  - let you manage networking
  - let you update packages
  - let you make networked backups
  - give you a virtual tty for the server
  - be able to control multiple of this (rust) server from one instance - worker/manager model
  - give data so i can have pretty graphs of resources that are available/allocated/in use