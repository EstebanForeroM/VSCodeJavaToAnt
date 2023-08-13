#VS Code Project Management Tool for Java Ant

Overview
This Rust program is designed to facilitate coding Java Ant projects in Visual Studio Code (VS Code) without relying on NetBeans. It provides a command-line interface for managing Java projects, including creating, opening, adding, deleting, and obtaining the Java Ant version for a project. The program leverages TOML for configuration and native dialog boxes for folder selection.

Features
Create a New Project: Copies a predefined VS Code configuration to a selected folder, renames it to the project name, and allows you to code Java Ant projects in VS Code.
Open a Project: Opens a selected project in VS Code.
Add a Project: Adds a new project to the configuration file.
Delete a Project: Removes a project from the configuration file.
Get Java Ant Version for a Project: Extracts the Java Ant version from a selected project, excluding specific directories, and combines it with a predefined Ant configuration.
Dependencies
native_dialog: For displaying file and folder selection dialogs.
serde: For serializing and deserializing the configuration.
toml: For working with TOML files.
fs_extra: For extra filesystem operations like copying directories.
Usage
Compile and run the program using Rust's standard tools. The main menu will be displayed, allowing you to select an option by entering a number.

Configuration
The program expects a TOML file named config.toml in the assets folder. This file should contain a mapping of project names to paths.

Example:

toml
Copy code
[proyectos]
"Project1" = "/path/to/project1"
"Project2" = "/path/to/project2"
Functions
create_project(): Handles the creation of a new project, enabling coding in VS Code.
open_project(): Opens a selected project in VS Code.
add_project(): Adds a new project to the configuration file.
delete_project(): Deletes a project from the configuration file.
get_java_ant(): Obtains the Java Ant version for a selected project.
Recommendation for Testing Java Ant Version
After obtaining the Java Ant version using the get_java_ant() function, it is recommended to open the resulting project with NetBeans and test it to ensure compatibility and correct functionality.

Error Handling
Each function returns a Result type, and the main function prints an error message if something goes wrong.

Conclusion
This tool simplifies the process of coding Java Ant projects in VS Code, providing a bridge between the flexibility of VS Code and the specific requirements of Java Ant. By facilitating the creation, management, and testing of projects, it offers a streamlined workflow for developers working with Java Ant.

Feel free to modify or expand this README as needed for your project!