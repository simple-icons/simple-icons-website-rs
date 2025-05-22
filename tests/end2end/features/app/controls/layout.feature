Feature: Change layout
	Ensures that the app loads the comfortable layout when it's opened
	and changes it by clicking on the buttons of the layout control.

	Background:
		Given I see the app

	Scenario: Load the comfortable layout and change it
		Then the selected layout is comfortable
		And the comfortable number of icons per page have been loaded
		When click on the compact button of the layout control
		Then the selected layout is compact
		And the compact number of icons per page have been loaded
		When click on the comfortable button of the layout control
		Then the selected layout is comfortable
