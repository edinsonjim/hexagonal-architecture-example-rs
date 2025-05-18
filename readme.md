# Hexagonal Architecture Example

In this project, you can see an example of how to organize a system using **hexagonal architecture**. I used this approach to keep the main business logic—called the **domain**—well structured.
You'll also notice some new ideas I'm working with: **queries**, **selectors**, and **valuables**. I'm planning to write a detailed explanation of each of these soon.

Organizing the domain this way has helped me clearly separate the different parts of the system. For example, when the domain needs to update the **repository**, I call those actions changes. And when something outside the domain asks for an action, I call those commands (yes, similar to the **CQRS pattern**).
