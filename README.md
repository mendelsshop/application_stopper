# Application_Stopper

## About:
This is rust program that will stop the application, if it you are using for a certain amount of time (ie: you got sidetracked with it).
Currently its hardcoded to stop Discord, after 2 hours per day.
It checks in two minutes intervals, if the application is running for more than and if it is, it add two minutes to the counter.
If the counter reaches the limit, it will ask the use if they are using for something important, if they are, it will wait another 5 minutes before asking you again.
If they are not, it will stop the application.

## Future Improvements:
- Support for stopping other applications (and possibly websites).
- Add a GUI promt to ask the user if they are using the application for something important (and pause the application untill thay answer the prompt).
- Command line arguments:
    - `-h`: Show help/usage.
    - `-q <time>`: Set the timelimt for all applications.
    - `-i <time>`: Set the interval for checking the applications.
    - `-w <time>`: Set the time limit for ussing the application for something important.
- Support for multiple applications having different time limits and intervals.
- Full GUI and CLI that support changing the time limits and intervals from within the app.

