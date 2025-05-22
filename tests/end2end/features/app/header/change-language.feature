Feature: Change language using the language selector
	Rest of the options to discover and save languages are tested by leptos-fluent.

	Background:
		Given I see the app
		Then the app is in English

	Scenario: Change language to Spanish
		When I click on the language selector
		Then I see the language selector
		And I select the language "Español"
		Then I don't see the language selector
		And the app is in Spanish
