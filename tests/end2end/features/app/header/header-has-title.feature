Feature: Header has title and description
	This is the most basic test to check if the app is running.

	Background:
		Given I see the app

	Scenario: The app has a title
		Then the title of the header is "Simple Icons"

	Scenario: The app has a description
		Then the description of the header includes "SVG icons for popular brands"
