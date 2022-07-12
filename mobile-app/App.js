import React from 'react';
import { StyleSheet, StatusBar, Text, View, Button } from 'react-native';
import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import { getHeaderTitle } from '@react-navigation/elements';

const backgroundColor = 'darkblue';
const textColor = '#f3f3f3';

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor,
    color: textColor,
    alignItems: 'center',
    justifyContent: 'center',
  },
});

const Stack = createNativeStackNavigator();

function WelcomeScreen({ navigation }) {
  return (
    <View style={styles.container}>
      <>
        <Text style={{ fontSize: 25, color: textColor }}>Welcome to Htracker</Text>
        <Button 
          title='Go To Login Screen'
          onPress={() => navigation.navigate('Login')}
        />
      </>
    </View>
  );
}

function LoginScreen({ navigation, back }) {
  return (
    <View style={styles.container}>
      <Text style={{ color: textColor }}>Login Here</Text>
    </View>
  );
}

export default function App() {
  return (
    <>
      <StatusBar style='auto'/>
      <NavigationContainer>
        <Stack.Navigator>
          <Stack.Screen
            name='Welcome'
            component={WelcomeScreen}
            options={{
              headerShown: false,
            }}
          />
          <Stack.Screen
            name='Login'
            component={LoginScreen}
            options={{
              headerStyle: {
                backgroundColor,
              },
              headerTintColor: textColor,
              headerShadowVisible: false,
              animation: 'slide_from_right',
            }}
          />
        </Stack.Navigator>
      </NavigationContainer>
    </>
  );
}
