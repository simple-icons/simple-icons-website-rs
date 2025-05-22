Feature: Change color scheme
	Ensures that the app changes the color scheme by clicking on
	the buttons of the color scheme control.

	Background:
		Given I see the app

	Scenario: Change color scheme to dark
		When click on the light button of the color scheme control
		Then the app background is light
		When click on the dark button of the color scheme control
		Then the app background is dark

	Scenario: Change color scheme to light
		When click on the dark button of the color scheme control
		Then the app background is dark
		When click on the light button of the color scheme control
		Then the app background is light
