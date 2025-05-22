Feature: Grid shows first page of icons on load
	Ensures that the app loads the first page of icons when it is opened.

	Background:
		Given I see the app
		And I see the grid

	Scenario: The app shows the first page of icons
		Then the default number of icons per page have been loaded
