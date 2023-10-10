# Hack Steps

1. Fetch the feedback page
2. Extract csrf token and session cookie
3. Inject payload into the name field when submitting a feedback to cause a 10 second delay
4. Wait for the response

# Run Script

1. Change the URL of the lab
2. Start script

```
~$ cargo run
```

# Expected Output

```
⟪#⟫ Injection point: name
⦗1⦘ Fetching the feedback page.. OK
⦗2⦘ Extracting csrf token and session cookie.. OK
⦗3⦘ Injecting payload to cause a 10 second delay (wait).. OK
🗹 Check your browser, it should be marked now as solved
```