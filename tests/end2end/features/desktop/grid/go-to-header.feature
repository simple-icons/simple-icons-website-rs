Feature: Go to header
	We go to footer, and then the "Go to header" button appears.
	We can go back to the header clicking on the "Go to header" button.
	Icons keeps loading when we go back to the header.

	Background:
		Given I see the app
		And I scroll to the top
		And the "Go to header" button does not exists

	Scenario: Go to footer clicking on the "Go to footer" button and then back
		Then the comfortable number of icons per page have been loaded
		When I click on the "Go to footer" button
		Then the footer touches the viewport
		When I click on the "Go to header" button
		Then the header touches the viewport
		Then the comfortable number of icons per page have been loaded

	Scenario: Keeps loading icons after return to header
		Then the comfortable number of icons per page have been loaded
		When I click on the "Go to footer" button
		Then the footer touches the viewport
		When I click on the "Go to header" button
		Then the header touches the viewport
		When I scroll to the footer
		Then the comfortable number of icons per page * 2 have been loaded
		When I scroll to the footer
		Then the comfortable number of icons per page * 3 have been loaded
