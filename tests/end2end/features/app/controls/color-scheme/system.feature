Feature: System color scheme
	Ensures that the app loads the system color scheme when it is opened.

	Background:
		Given I see the app

	Scenario: Load the system color scheme and change to it
		Then the app background is the system color scheme
		When click on the light button of the color scheme control
		Then the app background is light
		When click on the dark button of the color scheme control
		Then the app background is dark
		When click on the system button of the color scheme control
		Then the app background is the system color scheme
