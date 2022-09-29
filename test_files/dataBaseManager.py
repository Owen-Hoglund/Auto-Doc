from operator import itemgetter
import math
import datetime
from exampleimportfile import specificfunction
import flightProjectHelper as utility
import mysql.connector


# Opens the connection to our database
# ----------------------------------------------------------------------------------------------------------------------------------------------------
db = mysql.connector.connect(
    host = "localhost",
    user = "root",
    passwd = "ExamplePass",
    database = "testingenvironmentdatabase"
)
cursor = db.cursor(buffered=True)
# ----------------------------------------------------------------------------------------------------------------------------------------------------

# This initializes the necessary tables
# ----------------------------------------------------------------------------------------------------------------------------------------------------
def initalize_tables():
    # CREATES flightgroup table
    sqlstring = "CREATE TABLE if not exists flightgroups (email VARCHAR(50) NOT NULL, date_lower_bound datetime NOT NULL, date_upper_bound datetime NOT NULL, destination VARCHAR(3) NOT NULL, stayRangeLow int NOT NULL, stayRangeHigh int NOT NULL, PRIMARY KEY(email))"
    cursor.execute(sqlstring)

    # Creates flight_group_airport_list table
    sqlstring = "CREATE TABLE if not exists flight_group_airport_list (email VARCHAR(50) NOT NULL, origin_airport VARCHAR(3) NOT NULL, passenger_count int NOT NULL, FOREIGN KEY(email) REFERENCES flightgroups(email))"
    cursor.execute(sqlstring)
    
    # Creates flight_group_daily_price table
    sqlstring = "CREATE TABLE if not exists flight_group_daily_price (email VARCHAR(50) NOT NULL, date VARCHAR(50) NOT NULL, price_departure int, price_return int, FOREIGN KEY(email) REFERENCES flightgroups(email))"
    cursor.execute(sqlstring)
    
    # Creates iata_date_price table
    sqlstring = "CREATE TABLE if not exists iata_date_price (iata VARCHAR(50), date VARCHAR(50), price_departure int, price_return int, PRIMARY KEY (iata, date))"
    cursor.execute(sqlstring)
    sqlstring = "ALTER TABLE iata_iata_date_price CHANGE COLUMN `iata VARCHAR(50) NOT NULL, CHANGE COLUMN `date` `date` VARCHAR(50) NOT NULL ,ADD PRIMARY KEY (`iata, `date`)"
# ----------------------------------------------------------------------------------------------------------------------------------------------------


# The following block of code pushes all the data from a list of flightgroup objects (a 'handle') into the database
# ----------------------------------------------------------------------------------------------------------------------------------------------------
def groupTabler(handle):
    
    # Iterates over groups in handler
    for group in handle.groups:
        # converts time string to datetime
        low = utility.str_to_date_time(group.dateLowerBound)
        high = utility.str_to_date_time(group.dateUpperBound)
        email = group.email
        # inserts new group data row to the flightgroups table
        cursor.execute("INSERT INTO flightgroups VALUES (%s,%s,%s,%s,%s,%s)", (
            group.email, low, high, group.destination, group.stayRange[0], group.stayRange[1]))
        
        # iterates over the current groups origin list (in other words, how many people are departing from every given airport in the group)
        for origin_passengers in group.groupOrigins:
            # Inserts each airport-passenger tuple to the flight_group_airport_list 
            origin = str(origin_passengers[0])
            passengers = origin_passengers[1]
            sqlstring = "INSERT INTO flight_group_airport_list (email, origin_airport, passenger_count) VALUES ('" + email + "', '" + origin + "', " + str(passengers) + ")"
            cursor.execute(sqlstring)
            
        # Add dates into the flight_group_daily_price table for the current group, it defaults the price to be zero,
        # as updating prices happens in another part of the code
        for day in utility.daterange(low, high):
            sqlstring = "INSERT INTO flight_group_daily_price (email, date, price_departure, price_return) VALUES ('" + email + "', '" + utility.date_time_to_str(day) + "', 0, 0)"
            cursor.execute(sqlstring)
        
        # Add dates into the iata_date_price table for the current group
        for day in utility.daterange(low, high):
            for origins in group.groupOrigins:
                destination = group.destination
                itinerary_pair = utility.iata_pair(destination, origins[0])
                sqlstring = "INSERT IGNORE INTO iata_date_price (iata, date, price_departure, price_return) VALUES ('" + itinerary_pair + "', '" + utility.date_time_to_str(day) + "', 0, 0)"
                cursor.execute(sqlstring)
            
    # finalizes all of the above changes to the database. Conveniently 
    # if any errors occur in the above code none of these changes are
    # committed and you are left with a clean database still
    db.commit()
    # ----------------------------------------------------------------------------------------------------------------------------------------------------
    
    
    
# This block of code populates prices for each UNPRICED(=0) itinerary with a random integer
# ----------------------------------------------------------------------------------------------------------------------------------------------------
def populate_price_random_generator():
    # Fetch all unpriced itineraries from iata_date_price
    cursor.execute("SELECT * FROM iata_date_price WHERE price_return = 0 AND price_departure = 0")
    airport_to_airport_price = cursor.fetchall()
    # Update each itinerary with a random price
    for itinerary in airport_to_airport_price:
        itinerary_date = itinerary[1]
        cursor.execute("UPDATE iata_date_price SET price_departure=%s WHERE iata = %s AND date=%s", (utility.random_price_generator(), itinerary[0], itinerary_date))
        cursor.execute("UPDATE iata_date_price SET price_return=%s WHERE iata = %s AND date=%s", (utility.random_price_generator(), itinerary[0], itinerary_date))
    db.commit()
# ----------------------------------------------------------------------------------------------------------------------------------------------------
    
# This block of code sums up the total cost for a group to fly to and from their airports of origin and destination airport 
# on each day of the range of days they are interested in traveling on 
# ----------------------------------------------------------------------------------------------------------------------------------------------------
def populate_daily_price():
    print("Populating Daily Prices")
    
    # Get data from each flight group that we will update from
    sqlstring = "SELECT * FROM flightgroups"
    cursor.execute(sqlstring)
    groups = cursor.fetchall()
    
    # We will populate the prices group by group, so the first thing we do is iterate over each group
    for group in groups:
        # for simplicity, we will save all the static variables from each group below
        email = group[0]
        dateLow = group[1]
        dateHigh = group[2]
        destination = group[3]
        
        # Now we store the current groups origin list tuples. origin[1] is iata code origin[2] is the number of passengers traveling 
        # Now we need to store each origin airport-passenger count for calulating price later
        cursor.execute("SELECT * FROM flight_group_airport_list WHERE email = '" + group[0] + "'" )
        origin_list = cursor.fetchall()
        
        # Now we need to iterate over the days of travel. We do this using the generator in the flightProjectHelper
        for day in utility.daterange(dateLow, dateHigh):
            # The daily itinerary prices are stored in the iata_date_price table.
            # We do this by iterating over the itineraries that we have saved in the origin_list
            
            # Now we will, for each day in the range of dates, iterate over each itinerary in the origin airport-passenger count list we stored earlier
            cost_to_depart = 0     
            cost_to_return = 0      # We reset the price to zero for each new day to prevent prices from other dates leaking into the calculation
            
            for origin in origin_list:
                itinerary = utility.iata_pair(destination, origin[1])   # the itinerary pair (ABC-DEF) of airport codes keeps us from doing redundant calculations
                                                                        # by eliminating the need for calculating twice the price of an itinerary shared by multiple groups
                                                                              
                # Select all itineraries for the given iata pair on the current date we are calculating
                sqlstring = "SELECT * FROM iata_date_price WHERE iata = '%s' and date='%s'" % (itinerary, utility.date_time_to_str(day))
                cursor.execute(sqlstring)
                costs_for_day = cursor.fetchall()
                for costs in costs_for_day:
                    
                    # The itinerary pair (ABC-DEF) of airport codes keeps us from doing redundant calculations
                    # by eliminating the need for calculating twice the price of an itinerary shared by multiple groups
                    # as well as eliminating the need for two data points for two itineraries of the following form (MSP-BOS & BOS-MSP).
                    # We default the iata pair to alphabetical order, so it is important that the calculation is done correctly
                    # For example, price_departure for iata pair BOS-MSP is the price to fly from BOS to MSP. If the group is traveling 
                    # FROM MSP to BOS, the departure price will actually be the RETURN price in the database
                    if(utility.departure_or_return(destination, itinerary) == 'departure'):    # Departure or return will tell us which direction we need to calculate
                        # Add departure_price to cost to depart
                        cost_to_depart += costs[2] * origin[2]
                        cost_to_return += costs[3] * origin[2]
                    if(utility.departure_or_return(destination, itinerary) == 'return'):
                        # Add departure_price to cost to depart
                        cost_to_depart += costs[3] * origin[2]
                        cost_to_return += costs[2] * origin[2]
                    
            # Add cost_to_depart and cost_to_return to flight_group_daily_price
            sqlstring = "UPDATE flight_group_daily_price SET price_departure=%s WHERE email = '%s' AND date='%s'" % (cost_to_depart, email, utility.date_time_to_str(day))
            cursor.execute(sqlstring)
            sqlstring = "UPDATE flight_group_daily_price SET price_return=%s WHERE email = '%s' AND date='%s'" % (cost_to_return, email, utility.date_time_to_str(day))
            cursor.execute(sqlstring)
    db.commit()
# ----------------------------------------------------------------------------------------------------------------------------------------------------


# The final piece of the puzzle. We now need a table that gives the cheapest range of dates for a group to travel on. 
# First fetch group info to grab their date ranges
def best_dates(email):
    sqlstring = "SELECT * FROM flightgroups WHERE email='%s'" % email
    cursor.execute(sqlstring)
    group = cursor.fetchall()
    range_low = group[0][4]
    range_high = group[0][5]
    prices = [] # DepartDate, ReturnDate, Price
    for stay in range(range_low, range_high + 1):
        # get relevant daily prices
        sqlstring = "SELECT * FROM flight_group_daily_price  WHERE email = '%s' ORDER By date asc" % email
        cursor.execute(sqlstring)
        daily_prices = cursor.fetchall()

        for i in range(len(daily_prices)):
            if i + stay < len(daily_prices):
                price = daily_prices[i][2] + daily_prices[i+stay][3]
                prices.append((daily_prices[i][1], daily_prices[i+stay][1], price))
    sorted_prices = sorted(prices, key=itemgetter(2))
    return sorted_prices[0:5]