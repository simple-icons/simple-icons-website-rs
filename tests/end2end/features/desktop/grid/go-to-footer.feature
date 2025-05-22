Feature: Go to footer
	There is only one way to go to the footer: clicking on the "Go to footer"
	button. Because scrolling to the footer, the grid loads more icons.

	Background:
		Given I see the app
		And I scroll to the top
		And the "Go to header" button does not exists

	Scenario: Go to footer clicking on the "Go to footer" button
		Then the comfortable number of icons per page have been loaded
		When I click on the "Go to footer" button
		Then the footer touches the viewport
		Then the comfortable number of icons per page have been loaded

