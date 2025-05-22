Feature: Grid loads more icons when clicking button
	Ensures that the app loads the next page of icons when clicking
	to the "Load more icons" button.

	Background:
		Given I see the app
		And I see the grid

	Scenario: Load the next page clicking the "Load more icons" button
		Then the "Load more icons" button does not exists
		And the comfortable number of icons per page have been loaded

		When I click on the "Go to footer" button
		Then I see the "Load more icons" button
		When I click on the "Load more icons" button
		Then the comfortable number of icons per page * 2 have been loaded
		And the "Load more icons" button does not exists

		When I click on the "Go to footer" button
		Then I see the "Load more icons" button
		When I click on the "Load more icons" button
		Then the comfortable number of icons per page * 3 have been loaded
		And the "Load more icons" button does not exists

