Feature: Grid loads more icons when scrolling
	Ensures that the app loads the next page of icons when scrolling to the footer.

	Background:
		Given I see the app
		And I see the grid

	Scenario: Load the next pages of icons on scroll
		Then the comfortable number of icons per page have been loaded
		When I scroll to the footer
		Then the comfortable number of icons per page * 2 have been loaded
		When I scroll to the footer
		Then the comfortable number of icons per page * 3 have been loaded
		When I scroll to the footer
		Then the comfortable number of icons per page * 4 have been loaded
		When I scroll to the footer
		Then the comfortable number of icons per page * 5 have been loaded
